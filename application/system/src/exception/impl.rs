use google_gmail1::Error as GmailError;
use reqwest::StatusCode;
use sea_orm::DbErr;
use std::collections::HashMap;

use super::model::{Exception, ToErrorsDict, ToException};

impl ToException for validator::ValidationErrors {
    fn to_exception(&self) -> Exception {
        Exception {
            code: StatusCode::UNPROCESSABLE_ENTITY.as_u16(),
            message: "Validation error".to_string(),
            errors: self.to_eroors_dict(),
        }
    }
}

impl ToErrorsDict for validator::ValidationErrors {
    fn to_eroors_dict(&self) -> HashMap<String, Vec<String>> {
        self.field_errors()
            .iter()
            .map(|(field, errors)| {
                let field = field.to_string();
                let errors_dict = errors
                    .iter()
                    .map(|error| error.code.to_string())
                    .collect::<Vec<String>>();

                (field, errors_dict)
            })
            .collect()
    }
}

impl ToException for DbErr {
    fn to_exception(&self) -> Exception {
        Exception {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "Database error".to_string(),
            errors: self.to_eroors_dict(),
        }
    }
}

impl ToErrorsDict for DbErr {
    fn to_eroors_dict(&self) -> HashMap<String, Vec<String>> {
        let mut errors = HashMap::new();
        errors.insert("database".to_string(), vec![self.to_string()]);

        errors
    }
}

impl ToException for GmailError {
    fn to_exception(&self) -> Exception {
        Exception {
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            message: "Gmail error".to_string(),
            errors: self.to_eroors_dict(),
        }
    }
}

impl ToErrorsDict for GmailError {
    fn to_eroors_dict(&self) -> HashMap<String, Vec<String>> {
        let mut errors = HashMap::new();
        match self {
            GmailError::HttpError(e) => {
                errors.insert("HTTP Error".to_string(), vec![e.to_string()])
            }
            GmailError::UploadSizeLimitExceeded(a, b) => errors.insert(
                "Upload Size Limit Exceeded".to_string(),
                vec![format!("{}: {}", a, b)],
            ),
            GmailError::BadRequest(e) => {
                errors.insert("Bad Request".to_string(), vec![e.to_string()])
            }
            GmailError::MissingAPIKey => errors.insert(
                "Missing API Key".to_string(),
                vec!["missing api key".to_string()],
            ),
            GmailError::MissingToken(e) => {
                errors.insert("Missing Token".to_string(), vec![e.to_string()])
            }
            GmailError::Cancelled => {
                errors.insert("Cancelled".to_string(), vec!["cancelled".to_string()])
            }
            GmailError::FieldClash(e) => {
                errors.insert("Field Crash".to_string(), vec![e.to_string()])
            }
            GmailError::JsonDecodeError(a, b) => errors.insert(
                "GMAIL Json Decode Error".to_string(),
                vec![format!("{}: {}", a, b)],
            ),
            GmailError::Failure(_) => {
                errors.insert("Failure".to_string(), vec!["unexpected error".to_string()])
            }
            GmailError::Io(e) => errors.insert("IO Error".to_string(), vec![e.to_string()]),
        };

        errors
    }
}
