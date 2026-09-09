use crate::ApiError;

/// Standard API response wrapper for success and error cases.
///
/// Follows the JSend convention (`success` / `data` / `error`); see also
/// [`JSend`](crate::JSend) for the strict `{"status": ...}` JSend spelling
/// and [`ApiListResponse`] for paginated lists.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ApiResponse<T> {
    /// Whether the request was successful.
    pub success: bool,
    /// The response data. Present on success, absent on error.
    #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
    pub data: Option<T>,
    /// Error details. Present on failure, absent on success.
    #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
    pub error: Option<ApiError>,
    /// Pagination metadata. Present for list endpoints (see [`ApiResponse::paginated`]).
    #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
    pub pagination: Option<PaginationMeta>,
}

/// Pagination metadata for list responses.
///
/// Merged from the `json-envelope` crate so `api-types` is the single
/// envelope story.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct PaginationMeta {
    /// Current page number (1-indexed).
    pub page: u32,
    /// Number of items per page.
    pub per_page: u32,
    /// Total number of items across all pages.
    pub total: u64,
    /// Total number of pages.
    pub total_pages: u32,
}

impl<T> ApiResponse<T> {
    /// Creates a success response with the given data.
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            pagination: None,
        }
    }

    /// Creates a paginated success response with the given data and metadata.
    pub fn paginated(data: T, meta: PaginationMeta) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            pagination: Some(meta),
        }
    }

    /// Creates an error response.
    pub fn error(error: ApiError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            pagination: None,
        }
    }

    /// Creates an error response from a code and message.
    ///
    /// Shorthand for `ApiResponse::error(ApiError::new(code, message))`.
    /// (Merged from the `json-envelope` crate, where this was `error`.)
    pub fn error_with(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self::error(ApiError::new(code, message))
    }
}

impl<T> From<T> for ApiResponse<T> {
    /// Wraps data in a success response.
    fn from(data: T) -> Self {
        Self::success(data)
    }
}

/// API response for paginated lists.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ApiListResponse<T> {
    /// Whether the request was successful.
    pub success: bool,
    /// The list of items.
    #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
    pub data: Option<Vec<T>>,
    /// Current page number (1-indexed).
    #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
    pub page: Option<u32>,
    /// Number of items per page.
    #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
    pub per_page: Option<u32>,
    /// Total number of items across all pages.
    #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
    pub total: Option<u64>,
    /// Total number of pages.
    #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
    pub total_pages: Option<u64>,
    /// Error details. Present on failure.
    #[cfg_attr(feature = "serde_impl", serde(skip_serializing_if = "Option::is_none"))]
    pub error: Option<ApiError>,
}

impl<T> ApiListResponse<T> {
    /// Creates a success list response.
    pub fn success(items: Vec<T>, page: u32, per_page: u32, total: u64, total_pages: u64) -> Self {
        Self {
            success: true,
            data: Some(items),
            page: Some(page),
            per_page: Some(per_page),
            total: Some(total),
            total_pages: Some(total_pages),
            error: None,
        }
    }

    /// Creates an error list response.
    pub fn error(error: ApiError) -> Self {
        Self {
            success: false,
            data: None,
            page: None,
            per_page: None,
            total: None,
            total_pages: None,
            error: Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)] // test assertions unwrap by design
    use super::*;

    #[test]
    fn success_response() {
        let r = ApiResponse::success(42);
        assert!(r.success);
        assert_eq!(r.data, Some(42));
        assert!(r.error.is_none());
    }

    #[test]
    fn error_response() {
        let e = ApiError::new("BAD_REQUEST", "nope");
        let r: ApiResponse<()> = ApiResponse::error(e);
        assert!(!r.success);
        assert!(r.data.is_none());
        assert!(r.error.is_some());
    }

    #[test]
    fn error_with_response() {
        let r: ApiResponse<()> = ApiResponse::error_with("NOT_FOUND", "missing");
        assert!(!r.success);
        let err = r.error.unwrap();
        assert_eq!(err.code, "NOT_FOUND");
        assert_eq!(err.message, "missing");
    }

    #[test]
    fn paginated_response() {
        let meta = PaginationMeta {
            page: 2,
            per_page: 10,
            total: 25,
            total_pages: 3,
        };
        let r = ApiResponse::paginated(vec![1, 2], meta);
        assert!(r.success);
        assert_eq!(r.data.unwrap(), vec![1, 2]);
        let meta = r.pagination.unwrap();
        assert_eq!(meta.page, 2);
        assert_eq!(meta.total_pages, 3);
    }

    #[test]
    fn from_trait() {
        let r: ApiResponse<String> = ApiResponse::from("hello".to_string());
        assert!(r.success);
        assert_eq!(r.data.as_deref(), Some("hello"));
    }

    #[test]
    fn list_response() {
        let r = ApiListResponse::success(vec![1, 2, 3], 1, 10, 30, 3);
        assert!(r.success);
        assert_eq!(r.data.unwrap().len(), 3);
        assert_eq!(r.total, Some(30));
    }

    #[test]
    fn list_error_response() {
        let e = ApiError::new("INTERNAL_ERROR", "boom");
        let r: ApiListResponse<i32> = ApiListResponse::error(e);
        assert!(!r.success);
        assert!(r.data.is_none());
    }
}
