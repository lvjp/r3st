//! Shared library for the r3st binaries.

//! This crate intentionally stays transport-agnostic: it models the S3 data
//! types and error format so that both the server (which produces them) and the
//! conformance runner (which asserts on them) can speak the same language.

pub mod error;
pub mod model;

pub use error::{S3Error, S3ErrorCode};
pub use model::{Bucket, ListAllMyBucketsResult, Owner};
