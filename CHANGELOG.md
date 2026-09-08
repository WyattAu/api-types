# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

## [Unreleased]

### Added
- Merged `json-envelope` into `api-types`: `PaginationMeta`, `ApiResponse::paginated`,
  `ApiResponse::error_with`, `From<T> for ApiResponse<T>`, the proptest suite, the
  criterion benches (`benches/envelope_bench.rs`), and the fuzz target (`fuzz/`).
- New strict JSend envelope `JSend<T>` with `"status"` discriminator plus
  `From<ApiResponse<T>>` / `From<ApiError>` conversions.

### Changed
- `ApiResponse` gains an optional `pagination: Option<PaginationMeta>` field
  (struct literals that construct every field need updating).

## [0.1.0]

### Added
- Standard API response types — Success/Error/List wrappers, OpenAPI derives, RFC 7807 Problem Details.
- Not yet published to crates.io.
