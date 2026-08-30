# api-types

Standard API response types for Rust — Success, Error, and List wrappers with OpenAPI derives and RFC 7807 Problem Details.

## Overview

`api-types` provides reusable, strongly-typed response wrappers for JSON APIs. Instead of hand-rolling `{"success": true, "data": ...}` structs and error shapes, use `ApiResponse`, `ApiError`, and `ApiListResponse` to get consistent, serializable types across your services.

Includes an RFC 7807 `ProblemDetail` type for structured error responses that comply with the Problem Details for HTTP APIs standard.

## Features

- **`ApiResponse<T>`** — wrapper for success and error responses
- **`ApiError`** — structured error with code, message, and optional details
- **`ApiListResponse<T>`** — paginated list response wrapper
- **`ProblemDetail`** — RFC 7807 Problem Details for HTTP APIs
- **Axum integration** — `IntoResponse` implementation behind the `axum` feature
- **OpenAPI support** — optional `utoipa` derives behind the `openapi` feature
- **Serde support** — serialization/deserialization behind `serde_impl` (default)
- **No `unsafe`** — `#![forbid(unsafe_code)]`

## Usage

### Success response

```rust
use api_types::ApiResponse;

let resp = ApiResponse::success("hello world");
let json = serde_json::to_string(&resp).unwrap();
// {"success":true,"data":"hello world"}
```

### Error response

```rust
use api_types::{ApiResponse, ApiError};

let error = ApiError::new("NOT_FOUND", "Resource not found");
let resp = ApiResponse::<()>::error(error);
let json = serde_json::to_string(&resp).unwrap();
// {"success":false,"error":{"code":"NOT_FOUND","message":"Resource not found","details":null}}
```

### RFC 7807 Problem Detail

```rust
use api_types::ProblemDetail;

let problem = ProblemDetail::new("https://api.example.com/errors/not-found")
    .with_title("Not Found")
    .with_status(404)
    .with_detail("The requested resource does not exist.");
// Serializes to a JSON object conforming to RFC 7807
```

### Axum handler

```rust
use axum::Json;
use api_types::{ApiResponse, ApiError};

async fn get_item() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("item-data".into()))
}

async fn not_found() -> Json<ApiResponse<()>> {
    Json(ApiResponse::error(ApiError::new("NOT_FOUND", "Missing")))
}
```

## Comparison with manual responses

Without `api-types`, every endpoint needs ad-hoc structs:

```rust
// Manual — repetitive, inconsistent across services
#[derive(Serialize)]
struct SuccessResponse<T> { success: bool, data: T }

#[derive(Serialize)]
struct ErrorResponse { success: bool, error: ErrorBody }

#[derive(Serialize)]
struct ErrorBody { code: String, message: String }
```

`api-types` gives you a single set of canonical types with proper derives, OpenAPI schema generation, and RFC 7807 compliance out of the box.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
