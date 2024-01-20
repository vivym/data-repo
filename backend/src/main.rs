use std::net::{SocketAddr, IpAddr, Ipv4Addr};

use axum::http::HeaderValue;
use clap::Parser;
use tower_http::cors::AllowOrigin;

use backend::AppResult;
use backend::logger::setup_logging;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(default_value = "0.0.0.0", long, env)]
    hostname: String,
    #[clap(default_value = "8080", long, env)]
    port: u16,
    #[clap(long, env)]
    cors_allow_origin: Option<Vec<String>>,
    #[clap(long, env)]
    database_url: String,
    #[clap(long, env)]
    otlp_endpoint: Option<String>,
    #[clap(long, env)]
    json_log: bool,
}

#[tokio::main]
async fn main() -> AppResult<()> {
    match dotenvy::dotenv() {
        Ok(_) | Err(dotenvy::Error::Io(_)) => {},
        Err(e) => panic!("Failed to load .env file. Error: {:?}", e),
    };

    let args = Args::parse();

    let Args {
        hostname,
        port,
        cors_allow_origin,
        database_url,
        otlp_endpoint,
        json_log,
    } = args;

    setup_logging(otlp_endpoint, json_log);

    let cors_allow_origin = cors_allow_origin.map(|cors_allow_origin| {
        AllowOrigin::list(
            cors_allow_origin
                .iter()
                .map(|origin| HeaderValue::from_str(origin).unwrap()),
        )
    });

    let addr = match hostname.parse() {
        Ok(ip) => SocketAddr::new(ip, port),
        Err(_) => {
            tracing::warn!("Invalid hostname: {}, defaulting to 0.0.0.0", hostname);
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port)
        }
    };

    backend::server::run(
        addr,
        cors_allow_origin,
        database_url,
    ).await?;

    Ok(())
}
