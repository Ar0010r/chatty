use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::handler::{company, conversation, lead, manager};

pub fn all() -> Router {
    Router::new()
        .merge(lead())
        .merge(manager())
        .merge(company())
        .merge(conversation())
        .layer(middleware::from_fn(crate::middleware::auth))
        .route("/login", post(manager::login))
}

fn lead() -> Router {
    Router::new()
        .route("/lead", post(lead::create))
        .route("/lead/list", get(lead::list))
        .route("/lead/:id", get(lead::show))
        .route("/lead/:id", put(lead::update))
        .route("/lead/:id", delete(lead::delete))
}

fn manager() -> Router {
    Router::new()
        .route("/manager", post(manager::create))
        .route("/manager/list", get(manager::list))
        .route("/manager/:id", get(manager::show))
        .route("/manager/:id", put(manager::update))
        .route("/manager/:id", delete(manager::delete))
}

fn company() -> Router {
    Router::new()
        .route("/company", post(company::create))
        .route("/company/list", get(company::list))
        .route("/company/:id", get(company::show))
        .route("/company/:id", put(company::update))
        .route("/company/:id", delete(company::delete))
}

fn conversation() -> Router {
    Router::new()
        .route("/lead/write/:email", put(conversation::write_lead))
        .route("/lead/reply/:email", put(conversation::reply_lead))
        .route(
            "/lead/conversation/:email",
            get(conversation::get_conversation),
        )
}
