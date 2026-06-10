use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};

use crate::{models, routes};

/// Authorization: Bearer <JWT> 보안 스킴을 OpenAPI 컴포넌트에 등록한다.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi
            .components
            .as_mut()
            .expect("paths가 있으면 components는 항상 존재");
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Planner Backend API",
        version = "0.1.0",
        description = "Axum + SurrealDB 기반 플래너 API. 인증이 필요한 요청은 우측 상단 Authorize 버튼에 로그인/회원가입으로 받은 JWT를 넣고 'Try it out'으로 호출하세요."
    ),
    servers(
        (url = "http://localhost:8080", description = "로컬 개발 서버")
    ),
    paths(
        routes::user::register,
        routes::user::login,
        routes::user::me,
        routes::plan::list,
        routes::plan::create,
        routes::plan::get_one,
        routes::plan::update,
        routes::plan::delete,
        routes::photo::upload,
        routes::photo::serve,
    ),
    components(schemas(
        models::RegisterRequest,
        models::LoginRequest,
        models::AuthResponse,
        models::UserPublic,
        models::PlanPublic,
        models::CreatePlanRequest,
        models::UpdatePlanRequest,
        models::PhotoUpload,
    )),
    modifiers(&SecurityAddon),
    tags(
        (name = "auth", description = "회원가입 / 로그인 / 내 정보"),
        (name = "plans", description = "플랜 CRUD"),
        (name = "photos", description = "사진 업로드 / 서빙"),
    )
)]
pub struct ApiDoc;
