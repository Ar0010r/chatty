use axum::{extract::Request, http};

pub fn get_authorization(req: &Request) -> String {
    let auth_header = req.headers().get(http::header::AUTHORIZATION);

    match auth_header {
        Some(header) => header.to_str().unwrap_or_default().to_string(),
        None => String::default(),
    }
}
