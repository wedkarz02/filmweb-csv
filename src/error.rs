use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Unauthorized,
    BadRequest,
    InternalServerError,
    Unrecognized,
}

#[derive(Debug)]
pub enum AppError {
    ApiError(ApiError),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApiError(err) => write!(f, "{}", err),
            Self::ReqwestError(err) => write!(f, "reqwest error: {}", err),
            Self::SerdeJsonError(err) => write!(f, "serde_json error: {}", err),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(url) => write!(f, "Endpoint not found: {}", url),
            Self::Unauthorized => {
                write!(f, "Unauthorized, (missing cookie header)")
            }
            Self::BadRequest => write!(f, "Invalid or expired tokens"),
            Self::InternalServerError => write!(f, "Internal server error"),
            Self::Unrecognized => write!(f, "Unrecognized error"),
        }
    }
}

impl Error for ApiError {}

impl Error for AppError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AppError::ReqwestError(ref err) => Some(err),
            AppError::SerdeJsonError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<ApiError> for AppError {
    fn from(value: ApiError) -> Self {
        AppError::ApiError(value)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::ReqwestError(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::SerdeJsonError(value)
    }
}
