use axum::Router;
use std::env;

use crate::router;

pub async fn run() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .is_test(true)
        .init();

    let app = Router::new().merge(router::all());
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
