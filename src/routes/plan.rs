use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    AppState,
    auth::AuthUser,
    error::{AppError, Result},
    models::{CreatePlanRequest, Plan, UpdatePlanRequest},
};

#[derive(Deserialize)]
pub struct DateFilter {
    pub date: Option<String>,
    pub month: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Query(filter): Query<DateFilter>,
) -> Result<Json<Vec<Plan>>> {
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

    Ok(Json(plans))
}

pub async fn create(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(req): Json<CreatePlanRequest>,
) -> Result<Json<Plan>> {
    let plan: Option<Plan> = state
        .db
        .client
        .query(
            "CREATE plan SET
                user_id    = $uid,
                date       = <datetime> $date,
                title      = $title,
                content    = $content,
                photos     = [],
                created_at = time::now(),
                updated_at = time::now()",
        )
        .bind(("uid", claims.sub))
        .bind(("date", req.date.to_string()))
        .bind(("title", req.title))
        .bind(("content", req.content))
        .await?
        .take(0)?;

    plan.map(Json).ok_or_else(|| AppError::Internal(anyhow::anyhow!("플랜 생성 실패")))
}

pub async fn get_one(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
) -> Result<Json<Plan>> {
    let plan: Option<Plan> = state
        .db
        .client
        .query("SELECT * FROM plan WHERE id = $id AND user_id = $uid LIMIT 1")
        .bind(("id", format!("plan:{id}")))
        .bind(("uid", claims.sub))
        .await?
        .take(0)?;

    plan.map(Json)
        .ok_or_else(|| AppError::NotFound("플랜을 찾을 수 없습니다.".into()))
}

pub async fn update(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
    Json(req): Json<UpdatePlanRequest>,
) -> Result<Json<Plan>> {
    let existing: Option<Plan> = state
        .db
        .client
        .query("SELECT * FROM plan WHERE id = $id AND user_id = $uid LIMIT 1")
        .bind(("id", format!("plan:{id}")))
        .bind(("uid", claims.sub.clone()))
        .await?
        .take(0)?;

    existing.ok_or_else(|| AppError::NotFound("플랜을 찾을 수 없습니다.".into()))?;

    let plan: Option<Plan> = state
        .db
        .client
        .query(
            "UPDATE plan SET
                title      = if $title != NONE then $title else title end,
                content    = if $content != NONE then $content else content end,
                updated_at = time::now()
             WHERE id = $id AND user_id = $uid",
        )
        .bind(("id", format!("plan:{id}")))
        .bind(("uid", claims.sub))
        .bind(("title", req.title))
        .bind(("content", req.content))
        .await?
        .take(0)?;

    plan.map(Json)
        .ok_or_else(|| AppError::Internal(anyhow::anyhow!("플랜 업데이트 실패")))
}

pub async fn delete(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    let existing: Option<Plan> = state
        .db
        .client
        .query("SELECT * FROM plan WHERE id = $id AND user_id = $uid LIMIT 1")
        .bind(("id", format!("plan:{id}")))
        .bind(("uid", claims.sub.clone()))
        .await?
        .take(0)?;

    existing.ok_or_else(|| AppError::NotFound("플랜을 찾을 수 없습니다.".into()))?;

    state
        .db
        .client
        .query("DELETE plan WHERE id = $id AND user_id = $uid")
        .bind(("id", format!("plan:{id}")))
        .bind(("uid", claims.sub))
        .await?;

    Ok(Json(serde_json::json!({ "message": "삭제 완료" })))
}
