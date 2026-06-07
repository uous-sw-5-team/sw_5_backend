use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{error::AppError, models::Claims};

const SECRET: &[u8] = b"planner_secret_change_in_prod";
const EXPIRY_SECS: usize = 60 * 60 * 24 * 7; // 7일

pub fn create_token(user_id: &str) -> Result<String, AppError> {
    let exp = chrono::Utc::now()
        .timestamp() as usize
        + EXPIRY_SECS;

    let claims = Claims { sub: user_id.to_string(), exp };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|e| AppError::Auth(e.to_string()))
}

pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map(|d| d.claims)
    .map_err(|e| AppError::Auth(e.to_string()))
}

// Axum extractor: Authorization: Bearer <token> → Claims
pub struct AuthUser(pub Claims);

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({ "error": "Authorization 헤더 없음" })),
                )
            })?;

        verify_token(bearer.token()).map(AuthUser).map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({ "error": e.to_string() })),
            )
        })
    }
}
