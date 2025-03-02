use std::time::SystemTime;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct Metadata {
    success: bool,
    code: u16,
    message: String,
    timestamp: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct JsonResponse<T: Serialize + ToSchema> {
    pub data: T,
    pub meta: Metadata,
}

#[derive(Serialize, ToSchema)]
pub struct EmptyData {}

#[derive(Debug)]
pub struct ApiResponse<T: Serialize + ToSchema> {
    pub status_code: StatusCode,
    pub json: JsonResponse<T>,
}

pub struct ApiResponseBuilder<T> {
    data: T,
    success: bool,
    code: u16,
    message: String,
    status_code: StatusCode,
}

impl<T: Serialize + ToSchema> ApiResponseBuilder<T> {
    pub fn new(data: T) -> Self {
        ApiResponseBuilder {
            data,
            success: false,
            code: 0,
            message: "".into(),
            status_code: StatusCode::OK,
        }
    }

    pub fn status_code(mut self, status_code: StatusCode) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn code(mut self, code: u16) -> Self {
        self.code = code;
        self
    }

    pub fn message<S: Into<String>>(mut self, msg: S) -> Self {
        self.message = msg.into();
        self
    }

    pub fn build(self) -> ApiResponse<T> {
        ApiResponse {
            status_code: self.status_code,
            json: JsonResponse {
                data: self.data,
                meta: Metadata {
                    success: self.success,
                    code: self.code,
                    message: self.message,
                    timestamp: SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                },
            },
        }
    }


}

impl<T: Serialize + ToSchema> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> Response {
        (self.status_code, Json(self.json)).into_response()
    }
}