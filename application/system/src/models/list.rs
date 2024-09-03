use axum::{
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ListRequest<T> {
    pub filters: T,
    pub page: PageRequest,
    pub order_by: OrderBy,
    pub search_term: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OrderBy {
    pub column: String,
    pub direction: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PageRequest {
    pub page: u64,
    pub per_page: u64,
    pub order_by: Option<OrderBy>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub count: u64,
}

impl<T> List<T> {
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T: Serialize> IntoResponse for List<T> {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}
