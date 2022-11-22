use std::{collections::HashMap, convert::Into, str::FromStr};

use http_body_util::Empty;
use hyper::{
    body::{Body, Bytes},
    header::{HeaderName, IntoHeaderName},
    http::{uri::PathAndQuery, HeaderValue},
    Request, Response, Uri,
};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    net::TcpStream,
};

pub async fn connect_tcp_stream(url: &Uri) -> Result<TcpStream, anyhow::Error> {
    let host = url.host().expect("url has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    TcpStream::connect(addr).await.map_err(Into::into)
}

pub fn build_request(uri: &Uri) -> Result<Request<Empty<Bytes>>, anyhow::Error> {
    build_request_with_headers_and_body::<_, HeaderName>(
        uri,
        HashMap::default(),
        Empty::<Bytes>::new(),
    )
}

#[allow(unused)]
pub fn build_request_with_body<B>(uri: &Uri, body: B) -> Result<Request<B>, anyhow::Error>
where
    B: Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    build_request_with_headers_and_body::<B, HeaderName>(uri, HashMap::default(), body)
}

#[allow(unused)]
pub fn build_request_with_headers<K>(
    uri: &Uri,
    headers: HashMap<K, HeaderValue>,
) -> Result<Request<Empty<Bytes>>, anyhow::Error>
where
    K: IntoHeaderName,
{
    build_request_with_headers_and_body(uri, headers, Empty::<Bytes>::new())
}

pub fn build_request_with_headers_and_body<B, K>(
    uri: &Uri,
    headers: HashMap<K, HeaderValue>,
    body: B,
) -> Result<Request<B>, anyhow::Error>
where
    B: Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
    K: IntoHeaderName,
{
    let host = uri.host().expect("Host not found in uri").to_string();

    let mut request = Request::builder().uri(uri).body::<B>(body)?;

    let request_headers = request.headers_mut();

    // default host in case no headers are passed in but allow for overwriting
    request_headers.insert(hyper::header::HOST, HeaderValue::from_str(&host)?);

    for (k, v) in headers {
        request_headers.insert(k, v);
    }

    // headers.insert

    Ok(request)
}
pub async fn send_get_post<T, B>(
    stream: T,
    request: Request<B>,
) -> Result<Response<hyper::body::Incoming>, anyhow::Error>
where
    T: AsyncRead + AsyncWrite + Unpin + Send + 'static,
    B: Body + Send + 'static,
    B::Data: Send,
    B::Error: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    let (mut sender, conn) = hyper::client::conn::http1::handshake(stream).await?;

    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            panic!("Connection failed: {:?}", err);
        }
    });

    let response = sender.send_request(request).await?;

    Ok(response)
}

pub fn build_uri(base_url: Uri, path_and_query: &str) -> Result<Uri, anyhow::Error> {
    let mut parts = base_url.into_parts();

    parts.path_and_query =
        Some(PathAndQuery::from_str(path_and_query).expect("Invalid path and query"));

    Uri::from_parts(parts).map_err(Into::into)
}
