use axum::{body::Body, extract::Request, middleware::Next, response::Response};
use domain::{dto::manager::Manager, repositories::manager::ManagerReader};
use system::{
    exception::model::Exception,
    various::{
        http::get_authorization,
        jwt::{decode_or_default, strip_bearer},
    },
};

pub async fn auth(mut req: Request, next: Next) -> Result<Response<Body>, Exception> {
    let authorization = get_authorization(&req);
    let token = strip_bearer(&authorization);
    let token_data = decode_or_default(token);

    let mngr = ManagerReader::find_by_login(&token_data.claims.login).await?;
    let current_user = match mngr {
        Some(user) => Manager::from_model(user),
        None => Err(Exception::unathorized(String::default()))?,
    };

    req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
