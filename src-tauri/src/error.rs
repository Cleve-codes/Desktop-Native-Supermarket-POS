use serde::{Serialize, Deserialize};
use thiserror::Error;
use tauri::api::Error as TauriError;
use rusqlite::Error as RusqliteError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] RusqliteError),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Authorization error: {0}")]
    Authorization(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Tauri error: {0}")]
    Tauri(#[from] TauriError),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Other error: {0}")]
    Other(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let error_response = ErrorResponse {
            code: match self {
                AppError::Database(_) => "DATABASE_ERROR",
                AppError::Authentication(_) => "AUTHENTICATION_ERROR",
                AppError::Authorization(_) => "AUTHORIZATION_ERROR",
                AppError::Validation(_) => "VALIDATION_ERROR",
                AppError::NotFound(_) => "NOT_FOUND",
                AppError::Tauri(_) => "INTERNAL_ERROR",
                AppError::Serialization(_) => "SERIALIZATION_ERROR",
                AppError::Other(_) => "UNKNOWN_ERROR",
            }.to_string(),
            message: self.to_string(),
        };
        error_response.serialize(serializer)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub code: String,
    pub message: String,
}

pub type AppResult<T> = Result<T, AppError>; 