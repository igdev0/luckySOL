use std::env::VarError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Server error")]
    ServerError,
}