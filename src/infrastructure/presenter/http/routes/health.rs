use axum::extract::{State};
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use hyper::StatusCode;
use utoipa::{OpenApi};
use crate::state::AppState;
use crate::infrastructure::presenter::http::response::{ApiResponseBuilder};
use crate::infrastructure::presenter::http::response::JsonResponse;
use crate::infrastructure::presenter::http::routes::SecurityAddon;

const TAG: &str = "health";
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(health_check))
}

#[derive(OpenApi)]
#[openapi(
    paths(health_check),
    modifiers(&SecurityAddon),
    tags(
            (name = TAG, description = "Health Check")
    )
)]
pub struct ApiDoc;
#[utoipa::path(
        get,
        path = "/api/v1/health",
        tag = TAG,
        responses(
            (status = 200, description = "Health check", body = JsonResponse<String>)
        )
)]
pub async fn health_check(State(app_state): State<AppState>) -> impl IntoResponse {
    ApiResponseBuilder::new(app_state.service_name)
        .status_code(StatusCode::OK)
        .success(true)
        .code(20000)
        .message("success".to_string())
        .build()
}