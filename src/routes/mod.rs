pub mod user;
pub mod plan;
pub mod photo;

use axum::{Router, routing::{delete, get, post, put}};
use crate::AppState;

pub fn all_routes(state: AppState) -> Router {
    Router::new()
        // auth
        .route("/auth/register", post(user::register))
        .route("/auth/login",    post(user::login))
        .route("/auth/me",       get(user::me))
        // plans
        .route("/plans",         get(plan::list).post(plan::create))
        .route("/plans/{id}",    get(plan::get_one).put(plan::update).delete(plan::delete))
        // photos
        .route("/plans/{id}/photos", post(photo::upload))
        .route("/photos/{filename}",  get(photo::serve))
        .with_state(state)
}
