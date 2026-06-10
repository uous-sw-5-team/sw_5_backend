use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::{
    AppState,
    auth::AuthUser,
    error::{AppError, Result},
    models::{ErrorResponse, Plan, PlanPublic},
};

/// 사진 업로드
///
/// 플랜에 사진을 업로드합니다. multipart/form-data, 필드명 photo (여러 개 가능).
#[utoipa::path(
    post,
    path = "/api/plans/{id}/photos",
    tag = "photos",
    security(("bearer_auth" = [])),
    params(("id" = String, Path, description = "플랜 ID")),
    request_body(content = crate::models::PhotoUpload, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "photos 배열이 갱신된 플랜", body = PlanPublic),
        (status = 400, description = "지원하지 않는 형식 / 파일 없음", body = ErrorResponse),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "플랜 없음", body = ErrorResponse)
    )
)]
pub async fn upload(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(plan_id): Path<String>,
    mut multipart: Multipart,
) -> Result<Json<PlanPublic>> {
    let plan: Option<Plan> = state
        .db
        .client
        .query("SELECT * FROM plan WHERE id = type::thing('plan', $id) AND user_id = $uid LIMIT 1")
        .bind(("id", plan_id.clone()))
        .bind(("uid", claims.sub.clone()))
        .await?
        .take(0)?;

    let _ = plan.ok_or_else(|| AppError::NotFound("플랜을 찾을 수 없습니다.".into()))?;

    let mut saved_paths: Vec<String> = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| AppError::BadRequest(e.to_string()))? {
        let name = field.name().unwrap_or("").to_string();
        if name != "photo" {
            continue;
        }

        let content_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        let ext = match content_type.as_str() {
            "image/jpeg" => "jpg",
            "image/png"  => "png",
            "image/webp" => "webp",
            "image/gif"  => "gif",
            _ => return Err(AppError::BadRequest("지원하지 않는 이미지 형식입니다.".into())),
        };

        let filename = format!("{}.{}", Uuid::new_v4(), ext);
        let filepath = format!("uploads/{filename}");

        let data = field.bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?;
        let mut file = tokio::fs::File::create(&filepath).await?;
        file.write_all(&data).await?;

        saved_paths.push(filename);
    }

    if saved_paths.is_empty() {
        return Err(AppError::BadRequest("업로드된 파일이 없습니다.".into()));
    }

    let updated: Option<Plan> = state
        .db
        .client
        .query(
            "UPDATE plan SET
                photos     = array::concat(photos, $new_photos),
                updated_at = time::now()
             WHERE id = type::thing('plan', $id) AND user_id = $uid",
        )
        .bind(("id", plan_id))
        .bind(("uid", claims.sub))
        .bind(("new_photos", saved_paths))
        .await?
        .take(0)?;

    updated
        .map(|p| Json(PlanPublic::from(p)))
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("사진 저장 실패")))
}

/// 사진 서빙
///
/// 업로드된 이미지 파일을 반환합니다. 인증 불필요.
#[utoipa::path(
    get,
    path = "/api/photos/{filename}",
    tag = "photos",
    params(("filename" = String, Path, description = "plan.photos의 파일명")),
    responses(
        (status = 200, description = "이미지 바이너리", content_type = "image/*"),
        (status = 400, description = "잘못된 파일 이름", body = ErrorResponse),
        (status = 404, description = "파일 없음", body = ErrorResponse)
    )
)]
pub async fn serve(Path(filename): Path<String>) -> Result<Response> {
    if filename.contains("..") || filename.contains('/') {
        return Err(AppError::BadRequest("잘못된 파일 이름".into()));
    }

    let path = format!("uploads/{filename}");
    let data = tokio::fs::read(&path).await
        .map_err(|_| AppError::NotFound("파일을 찾을 수 없습니다.".into()))?;

    let content_type = match filename.rsplit('.').next().unwrap_or("") {
        "jpg" | "jpeg" => "image/jpeg",
        "png"          => "image/png",
        "webp"         => "image/webp",
        "gif"          => "image/gif",
        _              => "application/octet-stream",
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CACHE_CONTROL, "public, max-age=31536000")
        .body(Body::from(data))
        .unwrap())
}
