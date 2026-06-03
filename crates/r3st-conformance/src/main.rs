//! r3st-conformance: run a battery of checks against an S3 endpoint.
//!
//! Point it at any S3-compatible endpoint (including `r3st-api`) and it runs
//! each registered [`Check`], printing a pass/fail report and exiting non-zero
//! if anything failed — so it can be wired straight into CI.

mod checks;
mod runner;

use clap::Parser;
use runner::Runner;

/// Command-line options for the conformance runner.
#[derive(Debug, Parser)]
#[command(name = "r3st-conformance", version, about)]
struct Args {
    /// Base URL of the endpoint under test, e.g. `http://127.0.0.1:9000`.
    #[arg(long, env = "R3ST_ENDPOINT", default_value = "http://127.0.0.1:9000")]
    endpoint: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "r3st_conformance=info".into()),
        )
        .init();

    let args = Args::parse();
    let runner = Runner::new(&args.endpoint)?;

    let report = runner.run(checks::all()).await;
    report.print();

    if report.all_passed() {
        Ok(())
    } else {
        std::process::exit(1);
    }
}
