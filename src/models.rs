use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

// ── User ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPublic {
    pub id: String,
    pub username: String,
    pub email: String,
}

// ── Plan ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plan {
    pub id: Option<Thing>,
    pub user_id: String,
    pub date: NaiveDate,          // "YYYY-MM-DD"
    pub title: String,
    pub content: Option<String>,
    pub photos: Vec<String>,      // 파일 경로 목록
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlanRequest {
    pub date: NaiveDate,
    pub title: String,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePlanRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

// ── JWT Claims ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // user id
    pub exp: usize,
}
