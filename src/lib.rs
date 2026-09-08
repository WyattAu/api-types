#![forbid(unsafe_code)]
//! Standard API response types for Rust.
//!
//! Provides [`ApiResponse`], [`ApiError`], [`ApiListResponse`],
//! [`PaginationMeta`], [`JSend`], and [`ProblemDetail`] for building
//! consistent JSON APIs.
//!
//! This crate is the single envelope story for these APIs: it merges the
//! `json-envelope` crate's envelope (`ApiResponse` with [`PaginationMeta`],
//! `paginated`, `From` conversions), the JSend convention ([`JSend`]),
//! `utoipa` OpenAPI derives, and RFC 7807 Problem Details ([`ProblemDetail`]).
//! The `json-envelope` crate remains published as a thin re-export shim over
//! this crate.

mod error;
mod jsend;
mod problem;
mod response;

pub use error::ApiError;
pub use jsend::JSend;
pub use problem::ProblemDetail;
pub use response::{ApiListResponse, ApiResponse, PaginationMeta};
