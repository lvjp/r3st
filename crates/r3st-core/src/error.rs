//! The S3 REST error format.
//!
//! Amazon S3 reports failures as an XML body together with an HTTP status code.
//! See <https://docs.aws.amazon.com/AmazonS3/latest/API/ErrorResponses.html>.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// A subset of the documented S3 error codes.
///
/// Each variant carries the HTTP status code S3 associates with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum S3ErrorCode {
    AccessDenied,
    BucketAlreadyExists,
    BucketAlreadyOwnedByYou,
    BucketNotEmpty,
    InvalidBucketName,
    NoSuchBucket,
    NoSuchKey,
    InternalError,
}

impl S3ErrorCode {
    /// The HTTP status code S3 returns for this error.
    pub const fn http_status(self) -> u16 {
        match self {
            S3ErrorCode::AccessDenied => 403,
            S3ErrorCode::BucketAlreadyExists => 409,
            S3ErrorCode::BucketAlreadyOwnedByYou => 409,
            S3ErrorCode::BucketNotEmpty => 409,
            S3ErrorCode::InvalidBucketName => 400,
            S3ErrorCode::NoSuchBucket => 404,
            S3ErrorCode::NoSuchKey => 404,
            S3ErrorCode::InternalError => 500,
        }
    }

    /// The string form used in the `<Code>` element.
    pub const fn as_str(self) -> &'static str {
        match self {
            S3ErrorCode::AccessDenied => "AccessDenied",
            S3ErrorCode::BucketAlreadyExists => "BucketAlreadyExists",
            S3ErrorCode::BucketAlreadyOwnedByYou => "BucketAlreadyOwnedByYou",
            S3ErrorCode::BucketNotEmpty => "BucketNotEmpty",
            S3ErrorCode::InvalidBucketName => "InvalidBucketName",
            S3ErrorCode::NoSuchBucket => "NoSuchBucket",
            S3ErrorCode::NoSuchKey => "NoSuchKey",
            S3ErrorCode::InternalError => "InternalError",
        }
    }
}

/// An S3 error response, serializable to the XML format S3 clients expect.
#[derive(Debug, Clone, Error, Serialize, Deserialize)]
#[serde(rename = "Error")]
#[error("{code:?}: {message}")]
pub struct S3Error {
    #[serde(rename = "Code")]
    pub code: S3ErrorCode,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Resource", skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "RequestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl S3Error {
    /// Build an error with a default message for the given code.
    pub fn new(code: S3ErrorCode) -> Self {
        Self {
            code,
            message: code.as_str().to_string(),
            resource: None,
            request_id: None,
        }
    }

    /// Override the human-readable message.
    pub fn with_message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    /// Attach the offending resource (e.g. `/my-bucket/my-key`).
    pub fn with_resource(mut self, resource: impl Into<String>) -> Self {
        self.resource = Some(resource.into());
        self
    }

    /// The HTTP status code that should accompany this error body.
    pub fn http_status(&self) -> u16 {
        self.code.http_status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_codes_match_aws() {
        assert_eq!(S3ErrorCode::NoSuchBucket.http_status(), 404);
        assert_eq!(S3ErrorCode::BucketAlreadyExists.http_status(), 409);
        assert_eq!(S3ErrorCode::InvalidBucketName.http_status(), 400);
    }

    #[test]
    fn serializes_to_xml() {
        let err = S3Error::new(S3ErrorCode::NoSuchBucket).with_resource("/missing");
        let xml = quick_xml::se::to_string(&err).unwrap();
        assert!(xml.contains("<Code>NoSuchBucket</Code>"));
        assert!(xml.contains("<Resource>/missing</Resource>"));
    }
}
