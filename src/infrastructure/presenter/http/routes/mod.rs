use axum::{Router};
use utoipa::{
    openapi::security::{SecurityScheme},
    Modify, OpenApi,
};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;
use crate::state::AppState;

pub mod health;
pub mod todo;
pub struct SecurityAddon;
impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            )
        }
    }
}
pub fn init() -> Router<AppState> {

    let mut api_doc = health::ApiDoc::openapi();
    api_doc.merge(todo::ApiDoc::openapi());

    let api_router = Router::new()
        .nest("/health", health::router())
        .nest("/todo", todo::router());

    let router = Router::new()
        .nest("/api/v1", api_router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_doc.clone()))
        .merge(Redoc::with_url("/redoc", api_doc.clone()))
        .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"))
        .merge(Scalar::with_url("/scalar", api_doc));
    router
}