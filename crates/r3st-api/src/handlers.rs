//! HTTP handlers mapping S3 operations onto [`AppState`].

use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use r3st_core::S3Error;
use r3st_core::model::{Bucket, Buckets, ListAllMyBucketsResult, Owner};

use crate::state::AppState;

/// Render any value as an `application/xml` response body.
fn xml_response(status: StatusCode, body: &impl serde::Serialize) -> Response {
    match quick_xml::se::to_string(body) {
        Ok(xml) => (status, [(header::CONTENT_TYPE, "application/xml")], xml).into_response(),
        Err(err) => {
            tracing::error!(%err, "failed to serialize response");
            let fallback = S3Error::new(r3st_core::S3ErrorCode::InternalError);
            error_response(&fallback)
        }
    }
}

/// Render an [`S3Error`] with its associated HTTP status.
fn error_response(err: &S3Error) -> Response {
    let status =
        StatusCode::from_u16(err.http_status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    xml_response(status, err)
}

/// `GET /` — list all buckets owned by the caller.
///
/// <https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html>
pub async fn list_buckets(State(state): State<AppState>) -> Response {
    let buckets = state
        .list_buckets()
        .into_iter()
        .map(|(name, creation_date)| Bucket {
            name,
            creation_date,
        })
        .collect();

    let result = ListAllMyBucketsResult {
        owner: Owner {
            id: "r3st".into(),
            display_name: "r3st".into(),
        },
        buckets: Buckets { bucket: buckets },
    };

    xml_response(StatusCode::OK, &result)
}
