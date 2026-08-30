/// RFC 7807 Problem Details for HTTP APIs.
///
/// See <https://www.rfc-editor.org/rfc/rfc7807> for the specification.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde_impl", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "openapi", derive(utoipa::ToSchema))]
pub struct ProblemDetail {
    /// URI reference identifying the problem type.
    #[serde(rename = "type")]
    pub problem_type: String,
    /// Short human-readable summary.
    pub title: String,
    /// HTTP status code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    /// Human-readable explanation specific to this occurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// URI reference identifying the specific occurrence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl ProblemDetail {
    /// Creates a new problem detail with the given problem type URI.
    pub fn new(problem_type: impl Into<String>) -> Self {
        Self {
            problem_type: problem_type.into(),
            title: String::new(),
            status: None,
            detail: None,
            instance: None,
        }
    }

    /// Sets the title.
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the HTTP status code.
    pub fn with_status(mut self, status: u16) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the detail message.
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Sets the instance URI.
    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }

    /// Creates a "Not Found" problem detail.
    pub fn not_found() -> Self {
        Self::new("about:blank")
            .with_title("Not Found")
            .with_status(404)
    }

    /// Creates a "Bad Request" problem detail.
    pub fn bad_request() -> Self {
        Self::new("about:blank")
            .with_title("Bad Request")
            .with_status(400)
    }

    /// Creates an "Internal Server Error" problem detail.
    pub fn internal_error() -> Self {
        Self::new("about:blank")
            .with_title("Internal Server Error")
            .with_status(500)
    }
}

impl std::fmt::Display for ProblemDetail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.problem_type, self.title)
    }
}

impl std::error::Error for ProblemDetail {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_type() {
        let p = ProblemDetail::new("https://example.com/errors/too-hot");
        assert_eq!(p.problem_type, "https://example.com/errors/too-hot");
    }

    #[test]
    fn builder_chain() {
        let p = ProblemDetail::new("about:blank")
            .with_title("Not Found")
            .with_status(404)
            .with_detail("Widget 42 does not exist")
            .with_instance("/widgets/42");

        assert_eq!(p.title, "Not Found");
        assert_eq!(p.status, Some(404));
        assert_eq!(p.detail.as_deref(), Some("Widget 42 does not exist"));
        assert_eq!(p.instance.as_deref(), Some("/widgets/42"));
    }

    #[test]
    fn convenience_constructors() {
        let nf = ProblemDetail::not_found();
        assert_eq!(nf.status, Some(404));

        let br = ProblemDetail::bad_request();
        assert_eq!(br.status, Some(400));

        let ie = ProblemDetail::internal_error();
        assert_eq!(ie.status, Some(500));
    }

    #[test]
    fn serialize_skips_none_fields() {
        let p = ProblemDetail::new("about:blank").with_title("Oops");
        let json = serde_json::to_value(&p).unwrap();
        assert!(json.get("status").is_none());
        assert!(json.get("detail").is_none());
        assert!(json.get("instance").is_none());
    }
}
