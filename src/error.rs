use http::StatusCode;

/// A structured API error with code, message, and optional details.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ApiError {
    /// Machine-readable error code (e.g. `"NOT_FOUND"`).
    pub code: String,
    /// Human-readable error message.
    pub message: String,
    /// Optional additional details about the error.
    pub details: Option<serde_json::Value>,
}

impl ApiError {
    /// Creates a new API error with a code and message.
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    /// Attaches additional details to the error.
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = Some(details);
        self
    }

    /// Returns the appropriate HTTP status code for this error.
    pub fn status_code(&self) -> StatusCode {
        match self.code.as_str() {
            "BAD_REQUEST" | "VALIDATION_ERROR" => StatusCode::BAD_REQUEST,
            "UNAUTHORIZED" => StatusCode::UNAUTHORIZED,
            "FORBIDDEN" => StatusCode::FORBIDDEN,
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            "CONFLICT" => StatusCode::CONFLICT,
            "UNPROCESSABLE_ENTITY" => StatusCode::UNPROCESSABLE_ENTITY,
            "RATE_LIMITED" => StatusCode::TOO_MANY_REQUESTS,
            "INTERNAL_ERROR" => StatusCode::INTERNAL_SERVER_ERROR,
            "SERVICE_UNAVAILABLE" => StatusCode::SERVICE_UNAVAILABLE,
            _ => StatusCode::BAD_REQUEST,
        }
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let body = crate::ApiResponse::<()>::error(self);
        (status, axum::Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_error() {
        let e = ApiError::new("NOT_FOUND", "missing");
        assert_eq!(e.code, "NOT_FOUND");
        assert_eq!(e.message, "missing");
        assert!(e.details.is_none());
    }

    #[test]
    fn with_details() {
        let e =
            ApiError::new("BAD_REQUEST", "bad").with_details(serde_json::json!({"field": "name"}));
        assert!(e.details.is_some());
    }

    #[test]
    fn status_code_mapping() {
        assert_eq!(
            ApiError::new("NOT_FOUND", "").status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            ApiError::new("UNAUTHORIZED", "").status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            ApiError::new("INTERNAL_ERROR", "").status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
        assert_eq!(
            ApiError::new("UNKNOWN", "").status_code(),
            StatusCode::BAD_REQUEST
        );
    }

    #[test]
    fn display_impl() {
        let e = ApiError::new("CONFLICT", "already exists");
        assert_eq!(format!("{e}"), "[CONFLICT] already exists");
    }
}
