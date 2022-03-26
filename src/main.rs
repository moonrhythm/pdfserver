use std::{
    convert::Infallible,
    net::SocketAddr
};
use bytes::Buf;
use futures::StreamExt;
use chromiumoxide::{
    browser::{Browser, BrowserConfig},
    cdp::browser_protocol::page::PrintToPdfParams,
};
use hyper::{
    Body, header, Method, Request, Response, Server,
    service::{make_service_fn, service_fn}
};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_default()
        .parse()
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // let (browser, mut handler) = Browser::launch(
    //     BrowserConfig::builder()
    //         .build()?
    // ).await?;
    // tokio::spawn(async move {
    //     loop {
    //         let _ = handler.next().await.unwrap();
    //     }
    // });

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handler))
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
    scale: Option<f64>
}

async fn handler(req: Request<Body>) -> Result<Response<Body>> {
    if req.method() != &Method::POST {
        return Ok(Response::new("PDF Server".into()));
    }

    // TODO: improve error response

    let body = hyper::body::aggregate(req).await?;
    let data: PdfRequest = serde_json::from_reader(body.reader())?;

    // TODO: global browser ?
    let browser_config = BrowserConfig::builder()
        .build()?;
    let (browser, mut handler) = match Browser::launch(browser_config).await {
        Ok(r) => Ok(r),
        Err(e) => {
            eprintln!("can not launch browser, err={:?}", e);
            Err(e)
        }
    }?;
    tokio::spawn(async move {
        // loop {
        let _ = handler.next().await.unwrap();
        // }
    });

    let page = browser.new_page("about:blank").await?;
    page.set_content(data.content.as_str()).await?;

    let pdf_params = PrintToPdfParams{
        landscape: false.into(),
        display_header_footer: false.into(),
        print_background: true.into(),
        scale: data.scale,
        paper_width: None,
        paper_height: None,
        margin_top: None,
        margin_bottom: None,
        margin_left: None,
        margin_right: None,
        page_ranges: None,
        ignore_invalid_page_ranges: None,
        header_template: None,
        footer_template: None,
        prefer_css_page_size: None,
        transfer_mode: None
    };
    let pdf_data = page.pdf(pdf_params).await?;

    let resp = Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .body(Body::from(pdf_data))
        .unwrap();
    Ok(resp)
}
