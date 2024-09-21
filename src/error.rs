use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Unauthorized,
    BadRequest,
    InternalServerError,
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    Unrecognized,
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
            Self::ReqwestError(err) => write!(f, "reqwest error: {}", err),
            Self::SerdeJsonError(err) => write!(f, "serde_json error: {}", err),
            Self::Unrecognized => write!(f, "Unrecognized error"),
        }
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ApiError::ReqwestError(ref err) => Some(err),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(value: reqwest::Error) -> Self {
        ApiError::ReqwestError(value)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(value: serde_json::Error) -> Self {
        ApiError::SerdeJsonError(value)
    }
}
