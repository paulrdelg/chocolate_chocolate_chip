
use std::convert;
use std::fs;
use std::io;
use std::net;
use std::time;

use bytes;
use http_body_util;
use tokio;
use tracing;
use tracing_subscriber;

//use http_body_util::BodyExt;
//use http_body_util::combinators::UnsyncBoxBody;
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
use tokio_stream::StreamExt;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};


fn get_index() -> Bytes {
    let path = "src/client/index.html";
    let dat_res = fs::read(path);
    let dat = dat_res.unwrap();
    let data = Bytes::from(dat);

    tracing::info!("serving index.html");

    return data;
}

fn get_index_response() -> hyper::Response<http_body_util::combinators::UnsyncBoxBody<Bytes, convert::Infallible>> {
    let data = get_index();
    let full = http_body_util::Full::new(data);
    let boxed = http_body_util::BodyExt::boxed_unsync(full);

    hyper::Response::new(boxed)
}

fn get_stylecss() -> Bytes {
    let path = "src/client/style.css";
    let dat_res = fs::read(path);
    let dat = dat_res.unwrap();
    let data = Bytes::from(dat);

    tracing::info!("serving style.css");

    return data;
}

fn get_stylecss_response() -> hyper::Response<http_body_util::combinators::UnsyncBoxBody<Bytes, convert::Infallible>> {
    let data = get_stylecss();
    let full = http_body_util::Full::new(data);
    //let boxed = full.boxed();
    let boxed = http_body_util::BodyExt::boxed_unsync(full);

    hyper::Response::new(boxed)
}

fn get_appjs() -> Bytes {
    let path = "src/client/app.js";
    let dat_res = fs::read(path);
    let dat = dat_res.unwrap();
    let data = Bytes::from(dat);

    tracing::info!("serving app.js");

    return data;
}

fn get_appjs_response() -> hyper::Response<http_body_util::combinators::UnsyncBoxBody<Bytes, convert::Infallible>> {
    let data = get_appjs();
    let full = http_body_util::Full::new(data);
    //let boxed = full.boxed();
    let boxed = http_body_util::BodyExt::boxed_unsync(full);

    hyper::Response::new(boxed)
}

fn get_favicon() -> Bytes {
    let path = "src/client/favicon.ico";
    let dat_res = fs::read(path);
    let dat = dat_res.unwrap();
    let data = Bytes::from(dat);

    tracing::info!("serving favicon.ico");

    return data;
}

fn get_favicon_response() -> hyper::Response<http_body_util::combinators::UnsyncBoxBody<Bytes, convert::Infallible>> {
    let data = get_favicon();
    let full = http_body_util::Full::new(data);
    //let boxed = full.boxed();
    let boxed = http_body_util::BodyExt::boxed_unsync(full);

    hyper::Response::new(boxed)
}

fn get_events_response() -> hyper::Response<http_body_util::combinators::UnsyncBoxBody<Bytes, convert::Infallible>> {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<String>();

    tokio::spawn(async move {
        let mut i = 0u64;
        loop {
            i += 1;
            let msg = format!("tick #{i}");

            if tx.send(msg).is_err() {
                break;
            }

            tokio::time::sleep(time::Duration::from_secs(1)).await;
        }
    });

    let s = tokio_stream::wrappers::UnboundedReceiverStream::new(rx);

    let stream = s.map(|msg| {
        let line = format!("data: {msg}\n\n");
        let dat = bytes::Bytes::from(line);
        let frame = http_body::Frame::data(dat);
        Ok::<http_body::Frame<bytes::Bytes>, convert::Infallible>(frame)
    });

    let body = http_body_util::StreamBody::new(stream);
    let boxed = http_body_util::BodyExt::boxed_unsync(body);

    //let streambbody = http_body_util::StreamBody::new(stream);
    //let streamboxed = streambbody.boxed();

    hyper::Response::builder()
        .status(hyper::StatusCode::OK)
        .header(hyper::header::CONTENT_TYPE, "text/event-stream")
        .header("cache-control", "no-cache")
        .header("connection", "keep-alive")
        .body(boxed)
        .unwrap()
}

fn simple_text(status: hyper::StatusCode, text: &str) -> hyper::Response<http_body_util::combinators::UnsyncBoxBody<Bytes, convert::Infallible>> {
    let dat = Bytes::from(text.to_owned());
    let full = http_body_util::Full::from(dat);
    //let boxed = full.boxed();
    let boxed = http_body_util::BodyExt::boxed_unsync(full);

    let mut res = hyper::Response::new(boxed);
    *res.status_mut() = status;
    let hv = header::HeaderValue::from_static("text/plain; charset-utf-8");
    res.headers_mut().insert(header::CONTENT_TYPE, hv);

    return res;
}

async fn handler(req: hyper::Request<body::Incoming>) -> Result<hyper::Response<http_body_util::combinators::UnsyncBoxBody<Bytes, convert::Infallible>>, convert::Infallible> {
    let uri = req.uri();
    let path = uri.path();

    tracing::info!("request: {path}");

    let resp = match path {
        "/" => get_index_response(),
        "/app.js" => get_appjs_response(),
        "/style.css" => get_stylecss_response(),
        "/favicon.ico" => get_favicon_response(),
        "/events" => get_events_response(),
        _ => simple_text(hyper::StatusCode::NOT_FOUND, "not foundeded"),
    };

    Ok(resp)
}

fn get_ipv4_addr() -> net::Ipv4Addr {
    //let ip = [127, 0, 0, 1];
    let ip = net::Ipv4Addr::LOCALHOST;

    return ip;
}

fn get_socket_addr() -> net::SocketAddr {
    let ip = get_ipv4_addr();
    let port = 3000;
    let ip_addr = (ip, port);
    let addr = net::SocketAddr::from(ip_addr);

    return addr;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Hello, world!");

    //let t_env = tracing_subscriber::EnvFilter::from_default_env();
    let t_env = tracing_subscriber::EnvFilter::new("info");
    tracing_subscriber::registry()
        .with(t_env)
        .with(tracing_subscriber::fmt::layer().with_writer(io::stdout))
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
                tracing::error!("connection error from {peer}: {err}");
            }
        });
    }
}
