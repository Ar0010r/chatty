use axum::{
    async_trait,
    extract::{rejection::FormRejection, Form, FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationError};

use crate::exception::model::{Exception, ToException};

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedRequest<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedRequest<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = Exception;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| Exception::bad_request(Default::default()))?;
        match value.validate() {
            Ok(_) => Ok(ValidatedRequest(value)),
            Err(e) => Err(e.to_exception()),
        }
    }
}

pub struct Validator;

impl Validator {
    pub fn validate(value: impl Validate) -> Result<(), Exception> {
        match value.validate() {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_exception()),
        }
    }
}

pub fn validate_emails(emails: &[String]) -> Result<(), ValidationError> {
    let email_regex =
        regex::Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();

    if emails.is_empty() {
        return Err(ValidationError::new("emails"));
    }
    for email in emails.iter() {
        if !email_regex.is_match(email) {
            return Err(ValidationError::new("emails"));
        }
    }
    Ok(())
}
