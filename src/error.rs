use actix_web::ResponseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("db error")]
    DbError(#[from] sqlx::Error),

    #[error("not found")]
    NotFound,
}

impl ResponseError for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;
