
use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

async fn hello(_: hyper::Request<hyper::body::Incoming>) -> Result<hyper::Response<Full<Bytes>>, Infallible> {
    Ok(hyper::Response::new(Full::new(Bytes::from("Hello Test"))))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");
    let ip = [127, 0, 0, 1];
    let port = 3000;
    let addr = SocketAddr::from((ip, port));

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
            .serve_connection(io, service_fn(hello))
            .await 
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
