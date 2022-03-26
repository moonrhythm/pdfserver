use std::convert::Infallible;
use std::net::SocketAddr;
use bytes::Buf;
use headless_chrome::{Browser, protocol::page::PrintToPdfOptions};
use hyper::{Body, header, Method, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_default()
        .parse()
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handler))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("start pdf server on http://{}", addr);
    server.await?;

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct PdfRequest {
    content: String,
    scale: Option<f32>
}

async fn handler(req: Request<Body>) -> Result<Response<Body>> {
    if req.method() != &Method::POST {
        return Ok(Response::new("PDF Server".into()));
    }

    // TODO: improve error response

    let body = hyper::body::aggregate(req).await?;
    let data: PdfRequest = serde_json::from_reader(body.reader())?;
    let mut page_data: String = "data:text/html;base64,".to_owned();
    page_data.push_str(base64::encode(data.content).as_str());

    let browser = Browser::default()?;
    let tab = browser.wait_for_initial_tab()?;

    tab.navigate_to(page_data.as_str())?;
    tab.wait_until_navigated()?;

    let pdf_opt = PrintToPdfOptions{
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
        prefer_css_page_size: None
    };
    let pdf_data = tab.print_to_pdf(pdf_opt.into())?;

    let resp = Response::builder()
        .header(header::CONTENT_TYPE, "application/pdf")
        .body(Body::from(pdf_data))
        .unwrap();
    Ok(resp)
}
