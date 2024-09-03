use super::credentials::{self, DbCredentials};
use crate::exception::model::Exception;
use once_cell::sync::OnceCell;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

pub static PG_MAIN: OnceCell<DatabaseConnection> = OnceCell::new();

pub async fn init(creds: &DbCredentials) -> bool {
    let connection = establish(creds).await;
    PG_MAIN.set(connection).is_ok()
}

pub async fn retrieve() -> &'static DatabaseConnection {
    if PG_MAIN.get().is_none() {
        let connection = connect().await;
        let _ = PG_MAIN.set(connection);
    }

    get()
}

pub fn get() -> &'static DatabaseConnection {
    PG_MAIN.get().expect("Database connection not set")
}

pub async fn assure() -> Result<(), Exception> {
    if PG_MAIN.get().is_none() {
        let c = connect().await;
        return PG_MAIN
            .set(c)
            .map_err(|_| Exception::error("Failed to set connection".to_string()));
    }

    get()
        .ping()
        .await
        .map_err(|e| Exception::error(e.to_string()))
}

async fn connect() -> DatabaseConnection {
    establish(credentials::get()).await
}

pub async fn establish(creds: &DbCredentials) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(creds.get_connection_string());
    opt.max_connections(10)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        //.sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("public"); // Setting default PostgreSQL schema

    Database::connect(opt).await.unwrap()
}
