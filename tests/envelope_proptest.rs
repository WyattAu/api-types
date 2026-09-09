// Tests exercise serialization directly; unwrapping keeps assertions readable.
#![allow(clippy::unwrap_used)]

//! Property-based tests for the api-types envelope.
//!
//! Merged from the `json-envelope` crate's suite (adapted to the merged API:
//! `error_with` replaces `error(code, message)`), plus coverage for
//! `ApiListResponse` and the strict [`JSend`](api_types::JSend) envelope.

use api_types::{ApiError, ApiListResponse, ApiResponse, JSend, PaginationMeta};
use proptest::prelude::*;

proptest! {
    #[test]
    fn api_response_success_json_roundtrip(data in "[a-z]{1,100}") {
        let resp = ApiResponse::success(data.clone());
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["success"], serde_json::json!(true));
        assert_eq!(parsed["data"].as_str(), Some(data.as_str()));
    }

    #[test]
    fn api_response_error_json_roundtrip(
        code in "[a-z]{1,20}",
        message in "[a-z ]{1,100}",
    ) {
        let resp = ApiResponse::<()>::error_with(&code, &message);
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["success"], serde_json::json!(false));
        assert_eq!(parsed["error"]["code"].as_str(), Some(code.as_str()));
        assert_eq!(parsed["error"]["message"].as_str(), Some(message.as_str()));
    }

    #[test]
    fn api_response_paginated_json_roundtrip(
        data in "[a-z]{1,50}",
        page in 1u32..1000u32,
        per_page in 1u32..100u32,
        total in 0u64..1_000_000u64,
    ) {
        let total_pages = if per_page > 0 { ((total as f64 / per_page as f64).ceil() as u32).max(1) } else { 1 };
        let meta = PaginationMeta { page, per_page, total, total_pages };
        let resp = ApiResponse::paginated(data.clone(), meta);
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["success"], serde_json::json!(true));
        assert_eq!(parsed["data"].as_str(), Some(data.as_str()));
        assert_eq!(parsed["pagination"]["page"].as_u64(), Some(page as u64));
        assert_eq!(parsed["pagination"]["total"].as_u64(), Some(total));
    }

    #[test]
    fn api_response_from_trait(data in "[a-z]{1,100}") {
        let resp: ApiResponse<String> = data.clone().into();
        assert!(resp.success);
        assert_eq!(resp.data.as_deref(), Some(data.as_str()));
    }

    #[test]
    fn api_response_success_always_has_data(data in "[a-z]{1,100}") {
        let resp = ApiResponse::success(data);
        assert!(resp.success);
        assert!(resp.data.is_some());
        assert!(resp.error.is_none());
    }

    #[test]
    fn api_response_error_always_has_error(
        code in "[a-z]{1,20}",
        msg in "[a-z ]{1,100}",
    ) {
        let resp = ApiResponse::<()>::error_with(&code, &msg);
        assert!(!resp.success);
        assert!(resp.data.is_none());
        assert!(resp.error.is_some());
        let err = resp.error.unwrap();
        assert_eq!(err.code, code);
        assert_eq!(err.message, msg);
    }

    #[test]
    fn api_list_response_json_roundtrip(
        page in 1u32..100u32,
        per_page in 1u32..100u32,
        total in 0u64..10_000u64,
    ) {
        let total_pages = if per_page > 0 { (total / per_page as u64) + 1 } else { 1 };
        let resp = ApiListResponse::success(vec![1, 2, 3], page, per_page, total, total_pages);
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(&parsed["success"], &serde_json::json!(true));
        prop_assert_eq!(parsed["page"].as_u64(), Some(page as u64));
        prop_assert_eq!(parsed["total"].as_u64(), Some(total));
    }

    #[test]
    fn jsend_success_status_discriminator(data in "[a-z]{1,100}") {
        let resp = JSend::success(data.clone());
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(&parsed["status"], &serde_json::json!("success"));
        prop_assert_eq!(parsed["data"].as_str(), Some(data.as_str()));
    }

    #[test]
    fn jsend_error_status_discriminator(message in "[a-z ]{1,100}") {
        let resp = JSend::<()>::error(&message);
        let json = serde_json::to_string(&resp).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        prop_assert_eq!(&parsed["status"], &serde_json::json!("error"));
        prop_assert_eq!(parsed["message"].as_str(), Some(message.as_str()));
    }

    #[test]
    fn jsend_from_api_response_preserves_outcome(data in "[a-z]{1,100}") {
        let ok: JSend<String> = ApiResponse::success(data.clone()).into();
        prop_assert!(ok.is_success());

        let err: JSend<()> =
            ApiResponse::<()>::error(ApiError::new("NOT_FOUND", &data)).into();
        prop_assert!(!err.is_success());
    }
}
