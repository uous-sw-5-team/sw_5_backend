use axum::{extract::State, Json};

use crate::{
    AppState,
    auth::{AuthUser, create_token},
    error::{AppError, Result},
    models::{AuthResponse, LoginRequest, RegisterRequest, User, UserPublic},
};

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>> {
    let existing: Option<User> = state
        .db
        .client
        .query("SELECT * FROM user WHERE email = $email LIMIT 1")
        .bind(("email", req.email.clone()))
        .await?
        .take(0)?;

    if existing.is_some() {
        return Err(AppError::BadRequest("이미 사용 중인 이메일입니다.".into()));
    }

    let password_hash = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Auth(e.to_string()))?;

    let user: Option<User> = state
        .db
        .client
        .query(
            "CREATE user SET
                username      = $username,
                email         = $email,
                password_hash = $hash,
                created_at    = time::now()",
        )
        .bind(("username", req.username))
        .bind(("email", req.email))
        .bind(("hash", password_hash))
        .await?
        .take(0)?;

    let user = user.ok_or_else(|| AppError::Internal(anyhow::anyhow!("유저 생성 실패")))?;
    let id = user.id.as_ref().map(|t| t.to_string()).unwrap_or_default();
    let token = create_token(&id)?;

    Ok(Json(AuthResponse {
        token,
        user: UserPublic { id, username: user.username, email: user.email },
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let user: Option<User> = state
        .db
        .client
        .query("SELECT * FROM user WHERE email = $email LIMIT 1")
        .bind(("email", req.email))
        .await?
        .take(0)?;

    let user = user.ok_or_else(|| AppError::Auth("이메일 또는 비밀번호가 틀렸습니다.".into()))?;

    let valid = bcrypt::verify(&req.password, &user.password_hash)
        .map_err(|e| AppError::Auth(e.to_string()))?;

    if !valid {
        return Err(AppError::Auth("이메일 또는 비밀번호가 틀렸습니다.".into()));
    }

    let id = user.id.as_ref().map(|t| t.to_string()).unwrap_or_default();
    let token = create_token(&id)?;

    Ok(Json(AuthResponse {
        token,
        user: UserPublic { id, username: user.username, email: user.email },
    }))
}

pub async fn me(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<Json<UserPublic>> {
    let user: Option<User> = state
        .db
        .client
        .query("SELECT * FROM $id LIMIT 1")
        .bind(("id", claims.sub))
        .await?
        .take(0)?;

    let user = user.ok_or_else(|| AppError::NotFound("유저를 찾을 수 없습니다.".into()))?;
    let id = user.id.as_ref().map(|t| t.to_string()).unwrap_or_default();

    Ok(Json(UserPublic { id, username: user.username, email: user.email }))
}
