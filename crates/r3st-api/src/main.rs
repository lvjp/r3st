//! r3st-api: an S3-compatible HTTP server.
//!
//! This is a skeleton: it wires up the HTTP stack and a single working
//! operation (`ListBuckets`) so the conformance suite has something to talk to.
//! Fill in the remaining handlers under [`handlers`] as the implementation grows.

mod handlers;
mod state;

use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use clap::Parser;
use state::AppState;

/// Command-line options for the server.
#[derive(Debug, Parser)]
#[command(name = "r3st-api", version, about)]
struct Args {
    /// Address to bind to.
    #[arg(long, env = "R3ST_ADDR", default_value = "127.0.0.1:9000")]
    addr: SocketAddr,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "r3st_server=info,tower_http=info".into()),
        )
        .init();

    let args = Args::parse();
    let state = AppState::new();
    let app = router(state);

    let listener = tokio::net::TcpListener::bind(args.addr).await?;
    tracing::info!(addr = %args.addr, "r3st-api listening");
    axum::serve(listener, app).await?;
    Ok(())
}

/// Build the application router.
fn router(state: AppState) -> Router {
    Router::new()
        // GET Service — list all buckets.
        .route("/", get(handlers::list_buckets))
        .with_state(state)
}
