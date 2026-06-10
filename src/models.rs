use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;

// ── User ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<Thing>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    #[schema(example = "홍길동")]
    pub username: String,
    #[schema(example = "hong@example.com")]
    pub email: String,
    #[schema(example = "mypassword")]
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    #[schema(example = "hong@example.com")]
    pub email: String,
    #[schema(example = "mypassword")]
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserPublic,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
pub struct UserPublic {
    #[schema(example = "user:abc123")]
    pub id: String,
    #[schema(example = "홍길동")]
    pub username: String,
    #[schema(example = "hong@example.com")]
    pub email: String,
}

// ── Plan ──────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plan {
    pub id: Option<Thing>,
    pub user_id: String,
    pub date: NaiveDate,          // "YYYY-MM-DD"
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub time: Option<String>,     // 표시용 시각 문자열 (예: "오전 09:00")
    #[serde(default)]
    pub completed: bool,          // 완료 체크 여부
    pub photos: Vec<String>,      // 파일 경로 목록
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 프론트로 내보내는 형태: id를 평문 문자열("abc123")로 직렬화.
// 경로 파라미터(/plans/{id})에 그대로 다시 넣어 사용 가능.
#[derive(Debug, Serialize, ToSchema)]
pub struct PlanPublic {
    #[schema(example = "x7k2p9")]
    pub id: String,
    #[schema(example = "user:abc123")]
    pub user_id: String,
    #[schema(example = "2026-06-08")]
    pub date: NaiveDate,
    #[schema(example = "역사 에세이 초안 작성")]
    pub title: String,
    #[schema(example = "산업혁명의 영향에 관한 보고서 개요")]
    pub description: Option<String>,
    #[schema(example = "오전 09:00")]
    pub time: Option<String>,
    #[schema(example = false)]
    pub completed: bool,
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
            description: p.description,
            time: p.time,
            completed: p.completed,
            photos: p.photos,
            created_at: p.created_at,
            updated_at: p.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreatePlanRequest {
    #[schema(example = "2026-06-08")]
    pub date: NaiveDate,
    #[schema(example = "역사 에세이 초안 작성")]
    pub title: String,
    #[schema(example = "산업혁명의 영향에 관한 보고서 개요")]
    pub description: Option<String>,
    #[schema(example = "오전 09:00")]
    pub time: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePlanRequest {
    #[schema(example = "새 제목")]
    pub title: Option<String>,
    #[schema(example = "새 내용")]
    pub description: Option<String>,
    #[schema(example = "오후 03:00")]
    pub time: Option<String>,
    #[schema(example = true)]
    pub completed: Option<bool>,
}

/// multipart/form-data 사진 업로드 본문 (필드명: photo, 여러 개 가능)
#[derive(ToSchema)]
#[allow(dead_code)]
pub struct PhotoUpload {
    /// 업로드할 이미지 파일 (jpeg / png / webp / gif)
    #[schema(value_type = String, format = Binary)]
    pub photo: Vec<u8>,
}

// ── JWT Claims ────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // user id
    pub exp: usize,
}
