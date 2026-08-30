#![forbid(unsafe_code)]
//! Standard API response types for Rust.
//!
//! Provides [`ApiResponse`], [`ApiError`], [`ApiListResponse`], and
//! [`ProblemDetail`] for building consistent JSON APIs.

mod error;
mod problem;
mod response;

pub use error::ApiError;
pub use problem::ProblemDetail;
pub use response::{ApiListResponse, ApiResponse};
