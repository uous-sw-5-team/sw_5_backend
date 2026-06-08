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

// 프론트로 내보내는 형태: id를 평문 문자열("abc123")로 직렬화.
// 경로 파라미터(/plans/{id})에 그대로 다시 넣어 사용 가능.
#[derive(Debug, Serialize)]
pub struct PlanPublic {
    pub id: String,
    pub user_id: String,
    pub date: NaiveDate,
    pub title: String,
    pub content: Option<String>,
    pub photos: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Plan> for PlanPublic {
    fn from(p: Plan) -> Self {
        let id = p.id.map(|t| t.id.to_raw()).unwrap_or_default();
        PlanPublic {
            id,
            user_id: p.user_id,
            date: p.date,
            title: p.title,
            content: p.content,
            photos: p.photos,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
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
