use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not Found")]
    NotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}