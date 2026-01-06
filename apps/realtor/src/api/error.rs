use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    Forbidden(String),
    Conflict(String),
    TooManyRequests(String),
    Upstream(String),
    Internal(String),
}

impl ApiError {
    pub(crate) fn kind(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "bad_request",
            Self::Forbidden(_) => "forbidden",
            Self::Conflict(_) => "conflict",
            Self::TooManyRequests(_) => "too_many_requests",
            Self::Upstream(_) => "upstream",
            Self::Internal(_) => "internal",
        }
    }

    pub(crate) fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            Self::Upstream(_) => StatusCode::BAD_GATEWAY,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let kind = self.kind();
        let status = self.status_code();
        let msg = match self {
            Self::BadRequest(m)
            | Self::Forbidden(m)
            | Self::Conflict(m)
            | Self::TooManyRequests(m)
            | Self::Upstream(m)
            | Self::Internal(m) => m,
        };

        match status {
            s if s.is_server_error() => {
                tracing::warn!(%kind, status = s.as_u16(), error = %msg, "api error");
            }
            _ => {
                tracing::info!(%kind, status = status.as_u16(), error = %msg, "api error");
            }
        }

        (status, Json(serde_json::json!({ "error": msg }))).into_response()
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// Error message string.
    #[schema(example = "bad request")]
    pub error: String,
}
