use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::exception::model::Exception;

use super::list::List;

pub type ListResult<T> = Result<List<T>, Exception>;

pub type OptionResult<T> = Result<Option<T>, Exception>;

pub type VecResult<T> = Result<Vec<T>, Exception>;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Data<T>
where
    T: Serialize,
{
    pub data: T,
}

impl<T: Serialize> Data<T>
where
    T: Serialize,
{
    pub fn new(data: T) -> Self {
        Data { data }
    }
}

impl<T: Serialize> IntoResponse for Data<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
