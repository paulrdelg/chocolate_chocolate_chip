
use std::convert::Infallible;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper_util::server::conn::auto;
//use hyper::server::conn::http1;
//use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::rt::TokioExecutor;
//use hyper_util::support::TokioIo;
use tokio::net::TcpListener;

async fn hello(_: hyper::Request<hyper::body::Incoming>) -> Result<hyper::Response<Full<Bytes>>, Infallible> {
    Ok(hyper::Response::new(Full::new(Bytes::from("Hello Test"))))
}

//#[derive(Clone)]
//pub struct TokioExecutor;

//impl<F> hyper::rt::Executor<F> for TokioExecutor
//where
//    F: std::future::Future + Send + 'static,
//    F::Output: Send + 'static,
//{
//    fn execute(&self, fut: F) {
//        tokio::task::spawn(fut);
//    }
//}

fn get_ipv4_addr() -> Ipv4Addr {
    //let ip = [127, 0, 0, 1];
    let ip = Ipv4Addr::LOCALHOST;

    return ip;
}

fn get_socket_addr() -> SocketAddr {
    let ip = get_ipv4_addr();
    let port = 3000;
    let ip_addr = (ip, port);
    let addr = SocketAddr::from(ip_addr);

    return addr;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");

    let addr = get_socket_addr();

    let listener = TcpListener::bind(addr).await?;

    let builder = auto::Builder::new(TokioExecutor::new());

    loop {
        let (stream, peer) = listener.accept().await?;

        let io = TokioIo::new(stream);
        let svc = service_fn(hello);

        let builder = builder.clone();

        tokio::task::spawn(async move {
            let conn = builder.serve_connection(io, svc);

            if let Err(err) = conn.await {
                eprintln!("connection error from {peer}: {err}");
            }
        });
    }
}
