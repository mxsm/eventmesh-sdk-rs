use bytes::Bytes;
use http_body_util::Empty;
use crate::http::eventmesh_http_config::HttpConfig;
use crate::Messages::EventMeshMessage;
use hyper::{body::Buf, Request};
use tokio::net::TcpStream;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub struct EventMeshMessageHttpProuder {
    http_config: HttpConfig,
}

impl EventMeshMessageHttpProuder {
    fn publish(&self, eventmesh_message: &EventMeshMessage) {
        let uri = hyper::Uri::builder().scheme("https").build().unwrap();
        let message = Self::fetch_json(uri);
    }

    async fn fetch_json(url: hyper::Uri) -> Result<EventMeshMessage> {
        let host = url.host().expect("uri has no host");
        let port = url.port_u16().unwrap_or(80);
        let addr = format!("{}:{}", host, port);

        let stream = TcpStream::connect(addr).await?;
        let io = TokioIo::new(stream);
        let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;

        let authority = url.authority().unwrap().clone();
        let req = Request::builder()
            .uri(url)
            .header(hyper::header::HOST, authority.as_str())
            .body(Empty::<Bytes>::new())?;
        let res = sender.send_request(req).await?;

        let body = res.collect().await?.aggregate();

        let users = serde_json::from_reader(body.reader())?;

        Result::Ok(users)
    }
}

impl<T> TokioIo<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn inner(self) -> T {
        self.inner
    }
}

#[derive(Debug)]
pub struct TokioIo<T> {
    #[pin]
    inner: T,
}