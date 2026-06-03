//! S3 data types shared between the server and the conformance suite.

use serde::{Deserialize, Serialize};

/// The owner of a bucket or object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
}

/// A single bucket entry as returned by `ListBuckets`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(rename = "Name")]
    pub name: String,
    /// RFC 3339 timestamp, e.g. `2026-06-02T12:00:00.000Z`.
    #[serde(rename = "CreationDate")]
    pub creation_date: String,
}

/// The response body of the `ListBuckets` (GET Service) operation.
///
/// See <https://docs.aws.amazon.com/AmazonS3/latest/API/API_ListBuckets.html>.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename = "ListAllMyBucketsResult")]
pub struct ListAllMyBucketsResult {
    #[serde(rename = "Owner")]
    pub owner: Owner,
    #[serde(rename = "Buckets")]
    pub buckets: Buckets,
}

/// Wrapper element so the XML nests as `<Buckets><Bucket/>…</Buckets>`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Buckets {
    #[serde(rename = "Bucket", default)]
    pub bucket: Vec<Bucket>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_buckets_round_trips() {
        let result = ListAllMyBucketsResult {
            owner: Owner {
                id: "abc123".into(),
                display_name: "r3st".into(),
            },
            buckets: Buckets {
                bucket: vec![Bucket {
                    name: "demo".into(),
                    creation_date: "2026-06-02T12:00:00.000Z".into(),
                }],
            },
        };

        let xml = quick_xml::se::to_string(&result).unwrap();
        let parsed: ListAllMyBucketsResult = quick_xml::de::from_str(&xml).unwrap();
        assert_eq!(result, parsed);
    }
}
