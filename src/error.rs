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
    Api(ApiError),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    WithContext(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Api(err) => write!(f, "{}", err),
            Self::Reqwest(err) => write!(f, "reqwest error: {}", err),
            Self::SerdeJson(err) => write!(f, "serde_json error: {}", err),
            Self::WithContext(err) => write!(f, "{}", err),
        }
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(url) => write!(f, "Endpoint not found: {}", url),
            Self::Unauthorized => {
                write!(f, "Unauthorized (missing or invalid tokens)")
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
            AppError::Reqwest(ref err) => Some(err),
            AppError::SerdeJson(ref err) => Some(err),
            AppError::Api(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<ApiError> for AppError {
    fn from(value: ApiError) -> Self {
        AppError::Api(value)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        AppError::Reqwest(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::SerdeJson(value)
    }
}
