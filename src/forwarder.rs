use std::future::Future;
use std::pin::Pin;
use bytes::Bytes;
use http_body_util::Full;
use hyper::body::Incoming;
use hyper::{Request, Response};
use hyper::service::Service;

#[derive(Debug, Clone)]
pub(crate) struct Forwarder {}

impl Service<Request<Incoming>> for Forwarder {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        fn mk_response(s: String) -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        }

        let res = mk_response("hello".to_string());

        Box::pin(async { res })
    }
}