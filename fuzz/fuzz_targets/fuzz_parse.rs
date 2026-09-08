#![no_main]

use api_types::{ApiError, ApiResponse, PaginationMeta};
use libfuzzer_sys::fuzz_target;

// Merged from the `json-envelope` crate: the envelope has no inbound parse
// path (`ApiResponse` deserializes trusted output), so this harness fuzzes the
// real untrusted surface instead — adversarial bytes parsed as
// `serde_json::Value` embedded into the envelope, then serialized — plus
// re-parsing the emitted JSON.
fuzz_target!(|data: &[u8]| {
    // Bound input so parse attempts stay fast.
    let data = &data[..data.len().min(64 * 1024)];
    let s = String::from_utf8_lossy(data);

    // Arbitrary bytes as JSON must be Err, never panic.
    let value: Result<serde_json::Value, _> = serde_json::from_slice(data);
    let _ = serde_json::from_str::<serde_json::Value>(&s);

    // Untrusted JSON embedded as response `details`/`data` must serialize
    // without panicking, and the output must itself be valid JSON.
    let value = value.unwrap_or(serde_json::Value::Null);
    let ok: ApiResponse<serde_json::Value> = ApiResponse::success(value.clone());
    let out = serde_json::to_string(&ok);
    if let Ok(text) = out {
        assert!(serde_json::from_str::<serde_json::Value>(&text).is_ok());
    }

    let err = ApiResponse::<serde_json::Value> {
        success: false,
        data: None,
        error: Some(ApiError::new(s.to_string(), s.to_string()).with_details(value)),
        pagination: None,
    };
    let out = serde_json::to_string(&err);
    if let Ok(text) = out {
        assert!(serde_json::from_str::<serde_json::Value>(&text).is_ok());
    }

    // Pagination fields driven by input must serialize without panicking.
    let paginated = ApiResponse::paginated(
        serde_json::Value::String(s.to_string()),
        PaginationMeta {
            page: u32::from(data.first().copied().unwrap_or(0)),
            per_page: u32::from(data.get(1).copied().unwrap_or(0)),
            total: data.len() as u64,
            total_pages: u32::from(data.get(2).copied().unwrap_or(0)),
        },
    );
    let out = serde_json::to_string(&paginated);
    if let Ok(text) = out {
        assert!(serde_json::from_str::<serde_json::Value>(&text).is_ok());
    }
});
