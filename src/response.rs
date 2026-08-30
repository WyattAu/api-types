use crate::ApiError;

/// Standard API response wrapper for success and error cases.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ApiResponse<T> {
    /// Whether the request was successful.
    pub success: bool,
    /// The response data. Present on success, absent on error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    /// Error details. Present on failure, absent on success.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiError>,
}

impl<T> ApiResponse<T> {
    /// Creates a success response with the given data.
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Creates an error response.
    pub fn error(error: ApiError) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
        }
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,
    /// Current page number (1-indexed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    /// Number of items per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    /// Total number of items across all pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u64>,
    /// Total number of pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<u64>,
    /// Error details. Present on failure.
    #[serde(skip_serializing_if = "Option::is_none")]
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
