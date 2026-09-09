//! Strict [JSend](https://github.com/omniti-labs/jsend) envelope.
//!
//! [`ApiResponse`](crate::ApiResponse) follows the JSend *convention*
//! (`success` / `data` / `error`); this module provides the letter-of-the-spec
//! spelling with a `"status"` discriminator (`"success"`, `"fail"`, `"error"`)
//! for services that need byte-level JSend compliance.

use crate::{ApiError, ApiResponse};

/// Strict JSend response envelope.
///
/// Serializes with a `"status"` discriminator per the JSend specification:
///
/// - success: `{"status":"success","data":...}`
/// - fail: `{"status":"fail","data":...}`
/// - error: `{"status":"error","message":...,"code"?,"data"?}`
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
#[cfg_attr(
    feature = "serde_impl",
    serde(tag = "status", rename_all = "lowercase")
)]
pub enum JSend<T> {
    /// The request succeeded. `data` holds the result.
    Success {
        /// The response payload.
        #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
        data: Option<T>,
    },
    /// The request failed (client error). `data` describes what was wrong.
    Fail {
        /// Details about the failure (e.g. validation errors).
        #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
        data: Option<T>,
    },
    /// The request errored (server error).
    Error {
        /// Human-readable error message (required by JSend).
        message: String,
        /// Optional numeric error code.
        #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
        code: Option<i32>,
        /// Optional payload.
        #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
        data: Option<T>,
    },
}

impl<T> JSend<T> {
    /// Creates a JSend success response.
    pub fn success(data: T) -> Self {
        Self::Success { data: Some(data) }
    }

    /// Creates a JSend fail response.
    pub fn fail(data: T) -> Self {
        Self::Fail { data: Some(data) }
    }

    /// Creates a JSend error response with a message.
    pub fn error(message: impl Into<String>) -> Self {
        Self::Error {
            message: message.into(),
            code: None,
            data: None,
        }
    }

    /// Creates a JSend error response with a message and numeric code.
    pub fn error_with_code(message: impl Into<String>, code: i32) -> Self {
        Self::Error {
            message: message.into(),
            code: Some(code),
            data: None,
        }
    }

    /// Returns `true` for [`JSend::Success`].
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success { .. })
    }
}

impl<T> From<ApiResponse<T>> for JSend<T> {
    /// Maps an [`ApiResponse`] onto JSend: success stays success, any error
    /// becomes a JSend error carrying the API error message.
    fn from(resp: ApiResponse<T>) -> Self {
        match (resp.success, resp.data, resp.error) {
            (true, data, _) => Self::Success { data },
            (false, data, Some(err)) => Self::Error {
                message: err.message,
                code: None,
                data,
            },
            (false, data, None) => Self::Error {
                message: String::new(),
                code: None,
                data,
            },
        }
    }
}

impl From<ApiError> for JSend<()> {
    /// Maps an [`ApiError`] onto a JSend error.
    fn from(err: ApiError) -> Self {
        Self::Error {
            message: err.message,
            code: None,
            data: None,
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)] // test assertions unwrap by design
    use super::*;

    #[test]
    fn constructors() {
        assert!(JSend::success(1).is_success());
        assert!(!JSend::fail(1).is_success());
        assert!(!JSend::<()>::error("boom").is_success());
        let coded = JSend::<()>::error_with_code("boom", 500);
        assert!(matches!(
            coded,
            JSend::Error {
                code: Some(500),
                ..
            }
        ));
    }

    #[test]
    fn from_api_response() {
        let ok: JSend<i32> = ApiResponse::success(7).into();
        assert!(matches!(ok, JSend::Success { data: Some(7) }));

        let fail: JSend<()> =
            ApiResponse::<()>::error(ApiError::new("NOT_FOUND", "missing")).into();
        assert!(matches!(fail, JSend::Error { .. }));
        if let JSend::Error { message, .. } = fail {
            assert_eq!(message, "missing");
        }
    }

    #[test]
    fn from_api_error() {
        let js: JSend<()> = ApiError::new("BAD_REQUEST", "bad").into();
        assert!(matches!(js, JSend::Error { .. }));
    }

    #[cfg(feature = "serde_impl")]
    #[test]
    fn serde_status_discriminator() {
        let ok = JSend::success(1);
        let json = serde_json::to_value(&ok).unwrap();
        assert_eq!(json["status"], serde_json::json!("success"));
        assert_eq!(json["data"], serde_json::json!(1));

        let fail = JSend::fail("why");
        let json = serde_json::to_value(&fail).unwrap();
        assert_eq!(json["status"], serde_json::json!("fail"));

        let err = JSend::<()>::error_with_code("boom", 500);
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["status"], serde_json::json!("error"));
        assert_eq!(json["message"], serde_json::json!("boom"));
        assert_eq!(json["code"], serde_json::json!(500));

        let back: JSend<i32> = serde_json::from_value(json).unwrap();
        assert!(matches!(
            back,
            JSend::Error {
                code: Some(500),
                ..
            }
        ));
    }

    #[cfg(feature = "serde_impl")]
    #[test]
    fn serde_error_roundtrip() {
        let err = JSend::<String>::error("boom");
        let json = serde_json::to_string(&err).unwrap();
        let back: JSend<String> = serde_json::from_str(&json).unwrap();
        assert!(matches!(back, JSend::Error { .. }));
    }
}
