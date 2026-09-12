//! Config-knob behavior matrix for api-types.
//!
//! Every public builder setter must OBSERVABLY change behavior: the table
//! below pairs a default with a configured value and asserts the serialized
//! output differs. A knob that cannot change any observable output is a bug
//! (see breaker's sliding_window_size incident).
//!
//! Gated on `serde_impl`: the observable channel for these knobs is the
//! wire format, which only exists under that feature.
#![cfg(feature = "serde_impl")]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use api_types::{ApiError, ApiResponse, ProblemDetail};
use serde_json::Value;

/// Serialize an error through the public response envelope — the channel
/// API consumers actually observe.
fn error_json(err: &ApiError) -> Value {
    serde_json::to_value(ApiResponse::<()>::error(err.clone())).unwrap()
}

/// Serialize a problem detail — the RFC 7807 wire format.
fn problem_json(p: &ProblemDetail) -> Value {
    serde_json::to_value(p).unwrap()
}

// --- ApiError::with_details ----------------------------------------------

#[test]
fn knob_with_details_changes_serialized_error() {
    let default = error_json(&ApiError::new("NOT_FOUND", "missing"));
    let configured = error_json(
        &ApiError::new("NOT_FOUND", "missing")
            .with_details(serde_json::json!({"resource_id": 42, "hint": "check the spelling"})),
    );

    assert_ne!(
        default, configured,
        "with_details must change the wire output"
    );
    assert_eq!(
        configured["error"]["details"],
        serde_json::json!({"resource_id": 42, "hint": "check the spelling"}),
        "configured details must appear verbatim in the envelope"
    );
}

// --- ProblemDetail::with_title -------------------------------------------

#[test]
fn knob_with_title_changes_serialized_problem() {
    let default = problem_json(&ProblemDetail::new("about:blank"));
    let configured = problem_json(&ProblemDetail::new("about:blank").with_title("Not Found"));

    assert_ne!(
        default, configured,
        "with_title must change the wire output"
    );
    assert_eq!(default["title"], "", "unset title serializes as empty");
    assert_eq!(configured["title"], "Not Found");
}

// --- ProblemDetail::with_status ------------------------------------------

#[test]
fn knob_with_status_changes_serialized_problem() {
    let default = problem_json(&ProblemDetail::new("about:blank"));
    let configured = problem_json(&ProblemDetail::new("about:blank").with_status(404));

    assert_ne!(
        default, configured,
        "with_status must change the wire output"
    );
    assert!(
        default.get("status").is_none(),
        "unset status must be skipped, not null"
    );
    assert_eq!(configured["status"], 404);
}

// --- ProblemDetail::with_detail ------------------------------------------

#[test]
fn knob_with_detail_changes_serialized_problem() {
    let default = problem_json(&ProblemDetail::new("about:blank"));
    let configured =
        problem_json(&ProblemDetail::new("about:blank").with_detail("Widget 42 is missing"));

    assert_ne!(
        default, configured,
        "with_detail must change the wire output"
    );
    assert!(default.get("detail").is_none());
    assert_eq!(configured["detail"], "Widget 42 is missing");
}

// --- ProblemDetail::with_instance ----------------------------------------

#[test]
fn knob_with_instance_changes_serialized_problem() {
    let default = problem_json(&ProblemDetail::new("about:blank"));
    let configured = problem_json(&ProblemDetail::new("about:blank").with_instance("/widgets/42"));

    assert_ne!(
        default, configured,
        "with_instance must change the wire output"
    );
    assert!(default.get("instance").is_none());
    assert_eq!(configured["instance"], "/widgets/42");
}

// --- ApiError::status_code mapping (behavior table, not a setter) --------

#[test]
fn error_code_to_status_mapping_table() {
    let cases: &[(&str, u16)] = &[
        ("BAD_REQUEST", 400),
        ("VALIDATION_ERROR", 400),
        ("UNAUTHORIZED", 401),
        ("FORBIDDEN", 403),
        ("NOT_FOUND", 404),
        ("CONFLICT", 409),
        ("UNPROCESSABLE_ENTITY", 422),
        ("RATE_LIMITED", 429),
        ("INTERNAL_ERROR", 500),
        ("SERVICE_UNAVAILABLE", 503),
        ("ANYTHING_ELSE", 400), // unknown codes fail closed to 400
    ];
    for (code, status) in cases {
        let got = ApiError::new(*code, "").status_code();
        assert_eq!(got.as_u16(), *status, "code {code} must map to {status}");
    }
}
