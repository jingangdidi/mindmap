use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::process::exit;

use tokio::net::TcpListener;
use tracing::{event, Level};
use tracing_subscriber::{self, fmt::time::LocalTime};

use mindmap::{
    parse_paras::PARAS,
    router::configure,
    ctrlc::wait_for_signal,
};

#[tokio::main]
async fn main() {
    // listen `ctrl-c`
    tokio::spawn(async {
        wait_for_signal().await;
        exit(1);
    });

    // Start tracing
    tracing_subscriber::fmt() // INFO, WARN, ERROR, https://github.com/tokio-rs/tracing/blob/master/examples/examples/hyper-echo.rs
        .with_max_level(Level::INFO)
        .with_timer(LocalTime::rfc_3339()) // local time, RFC 3339
        .init();
    event!(Level::INFO, "Running on http://{}:{}", PARAS.addr_str, PARAS.port);

    // addr and port
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(PARAS.addr[0], PARAS.addr[1], PARAS.addr[2], PARAS.addr[3])), PARAS.port);
    // create router
    let router = configure();
    // start TCP, can also: let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await?;
    let listener = match TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            println!("{}", e);
            exit(1);
        },
    };
    // start http
    if let Err(e) = axum::serve(listener, router.into_make_service()).await {
        println!("{}", e);
        exit(1);
    }
}
