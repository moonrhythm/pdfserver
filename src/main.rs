use std::{convert::Infallible, net::SocketAddr};
use std::sync::Arc;

use bytes::Buf;
use chromiumoxide::{browser::{Browser, BrowserConfig}, cdp::browser_protocol::page::PrintToPdfParams};
use futures::StreamExt;
use hyper::{Body, header, Method, Request, Response, Server, service::{make_service_fn, service_fn}};
use serde::{Deserialize, Serialize};
use tokio::sync;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_default()
        .parse()
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let browser_config = BrowserConfig::builder().build()?;
    let (browser, mut browser_handler) = Browser::launch(browser_config).await?;
    tokio::spawn(async move {
        loop {
            let _ = browser_handler.next().await.unwrap();
        }
    });
    let browser = Arc::new(browser);

    let make_svc = make_service_fn(move |_conn| {
        let browser = Arc::clone(&browser);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handler(Arc::clone(&browser), req)
            }))
        }
    });
    let server = Server::bind(&addr)
        .serve(make_svc)
        .with_graceful_shutdown(shutdown_signal());

    println!("start pdf server on http://{}", addr);
    server.await?;
    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c().await.unwrap()
}

#[derive(Serialize, Deserialize)]
struct PdfRequest {
    content: String,
    scale: Option<f64>,
    paper: Option<PaperSize>,
}

#[derive(Serialize, Deserialize)]
struct PaperSize {
    width: Option<f64>,
    height: Option<f64>,
}

impl Default for PaperSize {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
        }
    }
}

async fn handler(browser: Arc<Browser>, req: Request<Body>) -> Result<Response<Body>> {
    if req.method() != Method::POST {
        return Ok(Response::new("PDF Server".into()));
    }

    let body = hyper::body::aggregate(req).await?;

    let data: PdfRequest = match serde_json::from_reader(body.reader()) {
        Ok(r) => r,
        Err(e) => {
            let resp = Response::builder()
                .status(400)
                .body(Body::from(format!("invalid request {:?}", e)))?;
            return Ok(resp);
        }
    };

    let paper = data.paper.unwrap_or_default();

    let pdf_params = PrintToPdfParams {
        landscape: false.into(),
        display_header_footer: false.into(),
        print_background: true.into(),
        scale: data.scale,
        paper_width: paper.width,
        paper_height: paper.height,
        margin_top: None,
        margin_bottom: None,
        margin_left: None,
        margin_right: None,
        page_ranges: None,
        ignore_invalid_page_ranges: None,
        header_template: None,
        footer_template: None,
        prefer_css_page_size: None,
        transfer_mode: None,
    };
    let pdf_data = match do_convert_pdf(browser, data.content, pdf_params).await {
        Ok(r) => r,
        Err(e) => {
            eprintln!("can not convert pdf, err={:?}", e);
            let resp = Response::builder()
                .status(500)
                .body(Body::from("can not convert pdf"))?;
            return Ok(resp);
        }
    };

    let resp = Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .body(Body::from(pdf_data))?;
    Ok(resp)
}

async fn do_convert_pdf(browser: Arc<Browser>,
                        content: String,
                        params: PrintToPdfParams,
) -> Result<Vec<u8>> {
    let (tx, rx) = sync::oneshot::channel();
    tokio::spawn(async move {
        let page = match browser.new_page("about:blank").await {
            Ok(p) => p,
            Err(e) => {
                dbg!(e);
                return;
            }
        };
        match page.set_content(content).await {
            Ok(p) => p,
            Err(e) => {
                let _ = page.close().await;
                dbg!(e);
                return;
            }
        };
        let data = match page.pdf(params).await {
            Ok(p) => p,
            Err(e) => {
                dbg!(e);
                let _ = page.close().await;
                return;
            }
        };
        let _ = page.close().await;
        let _ = tx.send(data);
    });
    Ok(rx.await?)
}
