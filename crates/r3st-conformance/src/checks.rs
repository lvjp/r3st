//! The battery of conformance checks.
//!
//! Add new checks here and register them in [`all`]. Each check is a small
//! `async fn` that talks to the endpoint through the [`Context`] and uses
//! `anyhow::ensure!` / `?` to report failures.

use anyhow::Context as _;
use r3st_core::model::ListAllMyBucketsResult;

use crate::runner::{Check, CheckFuture, Context};

/// All checks the runner should execute, in order.
pub fn all() -> Vec<Check> {
    vec![Check {
        name: "list_buckets_returns_valid_xml",
        run: |ctx| Box::pin(list_buckets_returns_valid_xml(ctx)) as CheckFuture<'_>,
    }]
}

/// `GET /` must return `200` with a parseable `ListAllMyBucketsResult` body.
async fn list_buckets_returns_valid_xml(ctx: &Context) -> anyhow::Result<()> {
    let resp = ctx
        .client
        .get(ctx.url("/"))
        .send()
        .await
        .context("sending ListBuckets request")?;

    anyhow::ensure!(
        resp.status().is_success(),
        "expected 2xx, got {}",
        resp.status()
    );

    let body = resp.text().await.context("reading ListBuckets body")?;
    let _: ListAllMyBucketsResult = quick_xml::de::from_str(&body)
        .with_context(|| format!("parsing ListBuckets body: {body}"))?;

    Ok(())
}
