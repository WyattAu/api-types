# api-types

[![docs.rs](https://docs.rs/api-types/badge.svg)](https://docs.rs/api-types)
[![crates.io](https://img.shields.io/crates/v/api-types.svg)](https://crates.io/crates/api-types)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Standard API response types for Rust — Success, Error, and List wrappers with OpenAPI derives and RFC 7807 Problem Details.

## Overview

`api-types` provides reusable, strongly-typed response wrappers for JSON APIs. Instead of hand-rolling `{"success": true, "data": ...}` structs and error shapes, use `ApiResponse`, `ApiError`, and `ApiListResponse` to get consistent, serializable types across your services.

Includes an RFC 7807 `ProblemDetail` type for structured error responses that comply with the Problem Details for HTTP APIs standard.

## Features

- **`ApiResponse<T>`** — wrapper for success and error responses
- **`ApiError`** — structured error with code, message, and optional details
- **`ApiListResponse<T>`** — paginated list response wrapper
- **`PaginationMeta`** — pagination metadata for list endpoints
- **`JSend<T>`** — strict [JSend](https://github.com/omniti-labs/jsend) envelope with `"status"` discriminator
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

### Paginated response

```rust
use api_types::{ApiResponse, PaginationMeta};

let meta = PaginationMeta { page: 1, per_page: 20, total: 1000, total_pages: 50 };
let resp = ApiResponse::paginated(items, meta);
// {"success":true,"data":[...],"pagination":{"page":1,...}}
```

### Strict JSend envelope

```rust
use api_types::JSend;

let resp = JSend::success("hello world");
// {"status":"success","data":"hello world"}

let err = JSend::<()>::error("Something went wrong");
// {"status":"error","message":"Something went wrong"}
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

## Relationship with `json-envelope`

`api-types` absorbs the `json-envelope` crate: `PaginationMeta`, `ApiResponse::paginated`, `ApiResponse::error_with`, the `From<T>` conversion, the proptest suite, the criterion benches, and the fuzz target all live here now. `json-envelope` remains published as a thin re-export shim over `api-types` so existing users don't break; new code should depend on `api-types` directly.

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
