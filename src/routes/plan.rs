use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    AppState,   
    auth::AuthUser,
    error::{AppError, Result},
    models::{CreatePlanRequest, ErrorResponse, Plan, PlanPublic, UpdatePlanRequest},
};

#[derive(Deserialize)]
pub struct DateFilter {
    pub date: Option<String>,
    pub month: Option<String>,
}

/// 플랜 목록 조회
///
/// 내 플랜 목록을 반환합니다. date 또는 month로 필터링할 수 있고, 없으면 전체를 최신 날짜순으로 반환합니다.
#[utoipa::path(
    get,
    path = "/api/plans",
    tag = "plans",
    security(("bearer_auth" = [])),
    params(
        ("date" = Option<String>, Query, description = "특정 날짜 (YYYY-MM-DD)"),
        ("month" = Option<String>, Query, description = "해당 월 (YYYY-MM)")
    ),
    responses(
        (status = 200, description = "플랜 목록", body = Vec<PlanPublic>),
        (status = 401, description = "인증 실패", body = ErrorResponse)
    )
)]
pub async fn list(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Query(filter): Query<DateFilter>,
) -> Result<Json<Vec<PlanPublic>>> {
    let plans: Vec<Plan> = if let Some(date) = filter.date {
        state
            .db
            .client
            .query("SELECT * FROM plan WHERE user_id = $uid AND date = $date ORDER BY created_at DESC")
            .bind(("uid", claims.sub))
            .bind(("date", date))
            .await?
            .take(0)?
    } else if let Some(month) = filter.month {
        let prefix = format!("{}-", month);
        state
            .db
            .client
            .query("SELECT * FROM plan WHERE user_id = $uid AND string::starts_with(string::from::date(date), $prefix) ORDER BY date ASC")
            .bind(("uid", claims.sub))
            .bind(("prefix", prefix))
            .await?
            .take(0)?
    } else {
        state
            .db
            .client
            .query("SELECT * FROM plan WHERE user_id = $uid ORDER BY date DESC")
            .bind(("uid", claims.sub))
            .await?
            .take(0)?
    };

    Ok(Json(plans.into_iter().map(PlanPublic::from).collect()))
}

/// 플랜 생성
///
/// 새 플랜을 생성합니다. completed는 항상 false로 시작합니다.
#[utoipa::path(
    post,
    path = "/api/plans",
    tag = "plans",
    security(("bearer_auth" = [])),
    request_body = CreatePlanRequest,
    responses(
        (status = 200, description = "생성된 플랜", body = PlanPublic),
        (status = 401, description = "인증 실패", body = ErrorResponse)
    )
)]
pub async fn create(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<CreatePlanRequest>,
) -> Result<Json<PlanPublic>> {
    let plan: Option<Plan> = state
        .db
        .client
        .query(
            "CREATE plan SET
                user_id     = $uid,
                date        = <string> $date,
                title       = $title,
                description = $description,
                `time`      = $time,
                completed   = false,
                photos      = [],
                created_at  = time::now(),
                updated_at  = time::now()",
        )
        .bind(("uid", claims.sub))
        .bind(("date", req.date.to_string()))
        .bind(("title", req.title))
        .bind(("description", req.description))
        .bind(("time", req.time))
        .await?
        .take(0)?;

    plan.map(|p| Json(PlanPublic::from(p)))
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("플랜 생성 실패")))
}

/// 플랜 단건 조회
///
/// id로 플랜 하나를 조회합니다. id는 목록/생성 응답의 평문 id를 그대로 사용합니다.
#[utoipa::path(
    get,
    path = "/api/plans/{id}",
    tag = "plans",
    security(("bearer_auth" = [])),
    params(("id" = String, Path, description = "플랜 ID (접두사 없는 평문)")),
    responses(
        (status = 200, description = "플랜 단건", body = PlanPublic),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "플랜 없음", body = ErrorResponse)
    )
)]
pub async fn get_one(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
) -> Result<Json<PlanPublic>> {
    let plan: Option<Plan> = state
        .db
        .client
        .query("SELECT * FROM plan WHERE id = type::thing('plan', $id) AND user_id = $uid LIMIT 1")
        .bind(("id", id))
        .bind(("uid", claims.sub))
        .await?
        .take(0)?;

    plan.map(|p| Json(PlanPublic::from(p)))
        .ok_or_else(|| AppError::NotFound("플랜을 찾을 수 없습니다.".into()))
}

/// 플랜 수정 (완료 토글 포함)
///
/// 보낸 필드만 부분 수정합니다. 완료 체크/해제는 completed 필드만 보내면 됩니다.
#[utoipa::path(
    put,
    path = "/api/plans/{id}",
    tag = "plans",
    security(("bearer_auth" = [])),
    params(("id" = String, Path, description = "플랜 ID")),
    request_body = UpdatePlanRequest,
    responses(
        (status = 200, description = "수정된 플랜", body = PlanPublic),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "플랜 없음", body = ErrorResponse)
    )
)]
pub async fn update(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
    Json(req): Json<UpdatePlanRequest>,
) -> Result<Json<PlanPublic>> {
    let existing: Option<Plan> = state
        .db
        .client
        .query("SELECT * FROM plan WHERE id = type::thing('plan', $id) AND user_id = $uid LIMIT 1")
        .bind(("id", id.clone()))
        .bind(("uid", claims.sub.clone()))
        .await?
        .take(0)?;

    existing.ok_or_else(|| AppError::NotFound("플랜을 찾을 수 없습니다.".into()))?;

    let plan: Option<Plan> = state
        .db
        .client
        .query(
            "UPDATE plan SET
                title       = if $title != NONE then $title else title end,
                description = if $description != NONE then $description else description end,
                `time`      = if $time != NONE then $time else `time` end,
                completed   = if $completed != NONE then $completed else completed end,
                updated_at  = time::now()
             WHERE id = type::thing('plan', $id) AND user_id = $uid",
        )
        .bind(("id", id))
        .bind(("uid", claims.sub))
        .bind(("title", req.title))
        .bind(("description", req.description))
        .bind(("time", req.time))
        .bind(("completed", req.completed))
        .await?
        .take(0)?;

    plan.map(|p| Json(PlanPublic::from(p)))
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("플랜 업데이트 실패")))
}

/// 플랜 삭제
///
/// id로 플랜을 삭제합니다.
#[utoipa::path(
    delete,
    path = "/api/plans/{id}",
    tag = "plans",
    security(("bearer_auth" = [])),
    params(("id" = String, Path, description = "플랜 ID")),
    responses(
        (status = 200, description = "삭제 완료 ({ \"message\": \"삭제 완료\" })"),
        (status = 401, description = "인증 실패", body = ErrorResponse),
        (status = 404, description = "플랜 없음", body = ErrorResponse)
    )
)]
pub async fn delete(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let existing: Option<Plan> = state
        .db
        .client
        .query("SELECT * FROM plan WHERE id = type::thing('plan', $id) AND user_id = $uid LIMIT 1")
        .bind(("id", id.clone()))
        .bind(("uid", claims.sub.clone()))
        .await?
        .take(0)?;

    existing.ok_or_else(|| AppError::NotFound("플랜을 찾을 수 없습니다.".into()))?;

    state
        .db
        .client
        .query("DELETE plan WHERE id = type::thing('plan', $id) AND user_id = $uid")
        .bind(("id", id))
        .bind(("uid", claims.sub))
        .await?;

    Ok(Json(serde_json::json!({ "message": "삭제 완료" })))
}
