use actix_web::ResponseError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("db error")]
    DbError(#[from] sqlx::Error),
}


impl ResponseError for AppError {}

pub type Result<T> = std::result::Result<T, AppError>;