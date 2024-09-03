use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use thiserror::Error;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Error)]
pub struct Exception {
    pub code: u16,
    pub message: String,
    pub errors: HashMap<String, Vec<String>>,
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

impl Exception {

    pub fn new(code: u16, message: String, errors: HashMap<String, Vec<String>>) -> Exception {
        Exception {
            code,
            message,
            errors,
        }
    }

    pub fn validation(errors: HashMap<String, Vec<String>>) -> Exception {
        Exception {
            code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            message: "Validation error".to_string(),
            errors,
        }
    }

    pub fn unathorized(message: String) -> Exception {
        Exception {
            errors: Default::default(),
            code: StatusCode::UNAUTHORIZED.as_u16(),
            message: match message.is_empty() {
                true => "Unathorized".to_string(),
                false => message,
            },
        }
    }

    pub fn bad_request(message: String) -> Exception {
        Exception {
            errors: Default::default(),
            code: StatusCode::BAD_REQUEST.as_u16(),
            message: match message.is_empty() {
                true => "Bad request".to_string(),
                false => message,
            },
        }
    }

    pub fn invalid_creds() -> Exception {
        Exception {
            errors: Default::default(),
            code: StatusCode::UNAUTHORIZED.as_u16(),
            message: "Invalid credemtials".to_string(),
        }
    }

    pub fn error(message: String) -> Exception {
        Exception {
            errors: Default::default(),
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: match message.is_empty() {
                true => "Internal server error".to_string(),
                false => message,
            },
        }
    }

    pub fn not_found(message: String) -> Exception {
        Exception {
            errors: Default::default(),
            code: StatusCode::NOT_FOUND.as_u16(),
            message: match message.is_empty() {
                true => "Resource not found".to_string(),
                false => message,
            },
        }
    }

    pub fn exists(param: String) -> Exception {
        Exception {
            code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            message: "Validation error".to_string(),
            errors: {
                let mut errors = HashMap::new();
                errors.insert(param.clone(), vec![format!("{} already exists", param)]);
                errors
            },
        }
    }
}

impl IntoResponse for Exception {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        )
            .into_response()
    }
}

pub trait ToException {
    fn to_exception(&self) -> Exception;
}

pub trait ToErrorsDict {
    fn to_eroors_dict(&self) -> HashMap<String, Vec<String>>;
}
