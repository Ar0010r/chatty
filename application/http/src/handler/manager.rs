use axum::{extract::Path, response::IntoResponse, Extension, Json};
use business::{actions::manager as action, various::auth::Credentials};
use domain::dto::manager::{Manager, ManagerBody, ManagerFilter, ManagerProps};
use system::{
    models::{list::ListRequest, shared::Data},
    various::auth,
};

#[axum::debug_handler]
pub async fn create(
    Extension(auth): Extension<Manager>,
    Json(payload): Json<ManagerBody>,
) -> impl IntoResponse {
    match action::create(payload, &auth).await {
        Ok(model) => Data::new(Manager::from_model(model)).into_response(),
        Err(exception) => exception.into_response(),
    }
}

#[axum::debug_handler]
pub async fn list(
    Extension(auth): Extension<Manager>,
    Json(payload): Json<ListRequest<ManagerFilter>>,
) -> impl IntoResponse {
    match action::list(payload, &auth).await {
        Ok(model) => model.into_response(),
        Err(exception) => exception.into_response(),
    };
}

#[axum::debug_handler]
pub async fn login(Json(payload): Json<Credentials>) -> impl IntoResponse {
    match auth::issue_token(payload).await {
        Ok(model) => Data::new(model).into_response(),
        Err(exception) => exception.into_response(),
    }
}

#[axum::debug_handler]
pub async fn show(Path(id): Path<i64>, Extension(auth): Extension<Manager>) -> impl IntoResponse {
    match action::show(id, &auth).await {
        Ok(model) => Data::new(model).into_response(),
        Err(exception) => exception.into_response(),
    }
}

#[axum::debug_handler]
pub async fn update(
    Path(id): Path<i64>,
    Extension(auth): Extension<Manager>,
    Json(payload): Json<ManagerProps>,
) -> impl IntoResponse {
    match action::update(id, payload, &auth).await {
        Ok(model) => Data::new(model).into_response(),
        Err(exception) => exception.into_response(),
    }
}

#[axum::debug_handler]
pub async fn delete(Path(id): Path<i64>, Extension(auth): Extension<Manager>) -> impl IntoResponse {
    match action::delete(id, &auth).await {
        Ok(model) => Data::new(model).into_response(),
        Err(exception) => exception.into_response(),
    }
}
