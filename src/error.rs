use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("DB 오류: {0}")]
    Db(#[from] surrealdb::Error),

    #[error("인증 오류: {0}")]
    Auth(String),

    #[error("찾을 수 없음: {0}")]
    NotFound(String),

    #[error("잘못된 요청: {0}")]
    BadRequest(String),

    #[error("서버 오류: {0}")]
    Internal(#[from] anyhow::Error),

    #[error("IO 오류: {0}")]
    Io(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Auth(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Db(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            AppError::Io(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };

        tracing::error!("[{}] {}", status, message);

        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
