use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub errors: Vec<ErrorDetail>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub long_message: String,
}

#[derive(Debug)]
pub enum ClientError {
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    InvalidRequest(String),
    ErrorResponse(ErrorResponse),
}

impl From<reqwest::Error> for ClientError {
    fn from(err: reqwest::Error) -> Self {
        ClientError::Reqwest(err)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClientError::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            ClientError::Json(e) => write!(f, "JSON error: {}", e),
            ClientError::InvalidRequest(e) => write!(f, "Invalid request: {}", e),
            ClientError::ErrorResponse(e) => write!(f, "Error response: {:?}", e.errors[0].message),
        }
    }
}
