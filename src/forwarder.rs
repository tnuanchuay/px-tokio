use bytes::Bytes;
use http_body_util::{BodyExt};
use http_body_util::combinators::BoxBody;
use hyper::{Request, Response};
use hyper::client::conn::http1::Builder;
use hyper::http::HeaderValue;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

pub async fn proxy(mut req: Request<hyper::body::Incoming>) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let stream = TcpStream::connect(("www.agoda.com", 80)).await.unwrap();
    let io = TokioIo::new(stream);

    let (mut sender, conn) = Builder::new()
        .preserve_header_case(true)
        .title_case_headers(true)
        .handshake(io)
        .await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    req.headers_mut()
        .insert("Host", HeaderValue::from_static("www.agoda.com"));

    println!("{:?}", req);

    let resp = sender.send_request(req).await?;
    Ok(resp.map(|b| b.boxed()))
}