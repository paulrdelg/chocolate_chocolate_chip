
use std::convert::Infallible;
use std::fs;
use std::net::Ipv4Addr;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body;
use hyper::body::Bytes;
use hyper::header;
use hyper_util::server::conn::auto;
//use hyper::server::conn::http1;
//use hyper::server::conn::http2;
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::rt::TokioExecutor;
//use hyper_util::support::TokioIo;
use tokio::net::TcpListener;
use tracing;
use tracing_subscriber;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


fn get_index() -> Bytes {
    let path = "src/client/index.html";
    let dat_res = fs::read(path);
    let dat = dat_res.unwrap();
    let data = Bytes::from(dat);

    println!("serving index");

    return data;
}

fn get_index_response() -> hyper::Response<Full<Bytes>> {
    let data = get_index();
    let full = Full::new(data);

    hyper::Response::new(full)
}

fn get_css() -> Bytes {
    let path = "src/client/style.css";
    let dat_res = fs::read(path);
    let dat = dat_res.unwrap();
    let data = Bytes::from(dat);

    println!("serving css");

    return data;
}

fn get_css_response() -> hyper::Response<Full<Bytes>> {
    let data = get_css();
    let full = Full::new(data);

    hyper::Response::new(full)
}

fn get_frontendapp() -> Bytes {
    let path = "src/client/app.js";
    let dat_res = fs::read(path);
    let dat = dat_res.unwrap();
    let data = Bytes::from(dat);

    println!("serving frontend app");

    return data;
}

fn get_app_response() -> hyper::Response<Full<Bytes>> {
    let data = get_frontendapp();
    let full = Full::new(data);

    hyper::Response::new(full)
}

fn simple_text(status: hyper::StatusCode, text: &str) -> hyper::Response<Full<Bytes>> {
    let mut res = hyper::Response::new(Full::from(Bytes::from(text.to_owned())));
    *res.status_mut() = status;
    let hv = header::HeaderValue::from_static("text/plain; charset-utf-8");
    res.headers_mut().insert(header::CONTENT_TYPE, hv);

    return res;
}

async fn handler(req: hyper::Request<body::Incoming>) -> Result<hyper::Response<Full<Bytes>>, Infallible> {
    let uri = req.uri();
    let path = uri.path();

    println!("request: {path}");

    let resp = match path {
        "/" => get_index_response(),
        "/app.js" => get_app_response(),
        "/style.css" => get_css_response(),
        _ => simple_text(hyper::StatusCode::NOT_FOUND, "not foundeded"),
    };

    Ok(resp)
}

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

    let t_env = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::registry()
        .with(t_env)
        .with(tracing_subscriber::fmt::layer())
        .init();

    let addr = get_socket_addr();

    let listener = TcpListener::bind(addr).await?;

    let builder = auto::Builder::new(TokioExecutor::new());

    loop {
        let (stream, peer) = listener.accept().await?;

        let io = TokioIo::new(stream);
        let svc = service_fn(handler);

        let builder = builder.clone();

        tokio::task::spawn(async move {
            let conn = builder.serve_connection(io, svc);

            if let Err(err) = conn.await {
                eprintln!("connection error from {peer}: {err}");
            }
        });
    }
}
