use axum::{extract::Path, response::IntoResponse, Extension, Json};
use business::actions::conversation as action;
use domain::dto::manager::Manager;
use system::{
    email::shared::models::{Body, SubjBody},
    models::shared::Data,
};

#[axum::debug_handler]
pub async fn get_conversation(
    Path(email): Path<String>,
    Extension(auth): Extension<Manager>,
) -> impl IntoResponse {
    match action::get_conversation(&email, auth).await {
        Ok(model) => Data::new(model).into_response(),
        Err(exception) => exception.into_response(),
    }
}

#[axum::debug_handler]
pub async fn write_lead(
    Path(email): Path<String>,
    Extension(auth): Extension<Manager>,
    Json(payload): Json<SubjBody>,
) -> impl IntoResponse {
    match action::write_to_lead(&email, payload, auth).await {
        Ok(model) => Data::new(model).into_response(),
        Err(exception) => exception.into_response(),
    }
}

#[axum::debug_handler]
pub async fn reply_lead(
    Path(email): Path<String>,
    Extension(auth): Extension<Manager>,
    Json(request): Json<Body>,
) -> impl IntoResponse {
    match action::reply_lead(&email.clone(), request.body, auth).await {
        Ok(model) => Data::new(model).into_response(),
        Err(exception) => exception.into_response(),
    }
}
