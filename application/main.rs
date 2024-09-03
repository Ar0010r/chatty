use system::{config, database::postgres as db};

#[tokio::main]
pub async fn main() {
    config::setup();
    db::connection::assure().await.unwrap();
    http::server::run().await;
}
