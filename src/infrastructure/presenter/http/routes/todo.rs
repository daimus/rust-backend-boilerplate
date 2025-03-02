use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::{Router, Json, middleware};
use axum::routing::{get};
use hyper::StatusCode;
use tower::ServiceBuilder;
use utoipa::{OpenApi};
use crate::common::app_error::AppError;
use crate::state::AppState;
use crate::infrastructure::presenter::http::response::{ApiResponseBuilder};
use crate::infrastructure::presenter::http::response::{JsonResponse, EmptyData};
use crate::infrastructure::presenter::http::routes::SecurityAddon;
use crate::infrastructure::presenter::http::entity::todo_dto::{TodoDto, CreateTodoDto, TodoList, TodoOption, UpdateTodoDto};
use crate::infrastructure::presenter::http::middleware::auth::auth;

const TAG: &str = "todo";
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_todos).post(create_todo))
        .route("/{id}", get(get_todo).patch(update_todo).delete(delete_todo))
        .layer(ServiceBuilder::new().layer(middleware::from_fn(auth)))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_todos,
        get_todo,
        create_todo,
        update_todo,
        delete_todo
    ),
    security(
        ("bearerAuth" = [])
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = TAG, description = "Todo Operations")
    )
)]
pub struct ApiDoc;
#[utoipa::path(
        get,
        path = "/api/v1/todo",
        tag = TAG,
        responses(
            (status = 200, description = "OK", body = JsonResponse<Vec<TodoDto>>),
            (status = 401, description = "Unauthorized", body = JsonResponse<String>),
            (status = 500, description = "Internal Server Error", body = JsonResponse<String>),
        )
)]
pub async fn get_todos(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let todos = state.dependencies.get_todos_usecase.execute().await;
    let result : Vec<TodoDto> = TodoList(todos).into();
    ApiResponseBuilder::new(result)
        .status_code(StatusCode::OK)
        .success(true)
        .code(20019)
        .message("success".to_string())
        .build()
}

#[utoipa::path(
        get,
        path = "/api/v1/todo/{id}",
        tag = TAG,
        params(
            ("id" = i32, Path, description = "Todo database id to get Todo for"),
        ),
        responses(
            (status = 200, description = "OK", body = JsonResponse<TodoDto>),
            (status = 401, description = "Unauthorized", body = JsonResponse<String>),
            (status = 404, description = "Not Found", body = JsonResponse<EmptyData>),
            (status = 500, description = "Internal Server Error", body = JsonResponse<String>),
        )
)]
pub async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let todo = state.dependencies.get_todo_usecase.execute(id).await;
    let result : Option<TodoDto> = TodoOption(todo).into();
    ApiResponseBuilder::new(result.clone())
        .status_code(match result.is_none() {
            true => StatusCode::NOT_FOUND,
            false => StatusCode::OK,
        })
        .success(true)
        .code(20019)
        .message("success".to_string())
        .build()
}

#[utoipa::path(
        post,
        path = "/api/v1/todo",
        tag = TAG,
        responses(
            (status = 201, description = "Created", body = JsonResponse<TodoDto>),
            (status = 401, description = "Unauthorized", body = JsonResponse<String>),
            (status = 500, description = "Internal Server Error", body = JsonResponse<String>),
        )
)]
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoDto>,
) -> impl IntoResponse {
    let create_todo_dto = payload.into();
    let todo = state.dependencies.create_todo_usecase.execute(create_todo_dto).await;
    let result : TodoDto = todo.into();
    ApiResponseBuilder::new(result)
        .status_code(StatusCode::CREATED)
        .success(true)
        .code(20101)
        .message("success".to_string())
        .build()
}

#[utoipa::path(
        patch,
        path = "/api/v1/todo/{id}",
        tag = TAG,
        params(
            ("id" = i32, Path, description = "Todo database id to update Todo for"),
        ),
        responses(
            (status = 200, description = "OK", body = JsonResponse<TodoDto>),
            (status = 401, description = "Unauthorized", body = JsonResponse<String>),
            (status = 500, description = "Internal Server Error", body = JsonResponse<String>),
        )
)]
pub async fn update_todo(
    State(state): State<AppState>,
    Path(todo_id): Path<i32>,
    Json(payload): Json<UpdateTodoDto>,
) -> impl IntoResponse {
    let update_todo_dto = payload.into();
    let todo = state.dependencies.update_todo_usecase.execute(todo_id, update_todo_dto).await;
    match todo {
        Err(AppError::NotFound) => return ApiResponseBuilder::new(None)
            .status_code(StatusCode::NOT_FOUND)
            .success(false)
            .code(40401)
            .message("todo not found".to_string())
            .build(),
        Err(AppError::Other(_)) => return ApiResponseBuilder::new(None)
            .status_code(StatusCode::INTERNAL_SERVER_ERROR)
            .success(false)
            .code(50001)
            .message("internal server error".to_string())
            .build(),
        _ => {}
    }
    let result : TodoDto = todo.unwrap().into();
    ApiResponseBuilder::new(Some(result))
        .status_code(StatusCode::OK)
        .success(true)
        .code(20101)
        .message("success".to_string())
        .build()
}

#[utoipa::path(
        delete,
        path = "/api/v1/todo/{id}",
        tag = TAG,
        params(
            ("id" = i32, Path, description = "Todo database id to delete Todo for"),
        ),
        responses(
            (status = 204, description = "OK"),
            (status = 401, description = "Unauthorized", body = JsonResponse<String>),
            (status = 500, description = "Internal Server Error", body = JsonResponse<String>),
        )
)]
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(todo_id): Path<i32>,
) -> impl IntoResponse { state.dependencies.delete_todo_usecase.execute(todo_id).await;
    ApiResponseBuilder::new(Some(Option::<TodoDto>::None))
        .status_code(StatusCode::NO_CONTENT)
        .success(true)
        .code(20401)
        .message("success".to_string())
        .build()
}