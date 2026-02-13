#![warn(
    clippy::nursery,
    clippy::pedantic,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::correctness,
    clippy::suspicious
)]

use chrono::Local;
use my_gateway::Gateway;
use pingora::prelude::*;
use std::fmt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

struct LoggerFormatter;

impl FormatTime for LoggerFormatter {
    fn format_time(&self, w: &mut Writer<'_>) -> fmt::Result {
        write!(w, "{}", Local::now().format("%Y-%m-%d %H:%M:%S"))
    }
}

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<()> {
    // Filter out Pingora's harmless broken pipe warnings
    // These occur when clients close connections early after receiving error responses
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,pingora_proxy::proxy_h1=info"));

    tracing_subscriber::fmt()
        .with_timer(LoggerFormatter)
        .with_env_filter(env_filter)
        .init();
    tracing::info!("Gateway starting with embedded static assets");

    let mut my_server = Server::new(None)?;
    my_server.bootstrap();

    let mut proxy_service = http_proxy_service(&my_server.configuration, Gateway::new());
    proxy_service.add_tcp("0.0.0.0:9066");

    my_server.add_service(proxy_service);
    my_server.run_forever();
}
