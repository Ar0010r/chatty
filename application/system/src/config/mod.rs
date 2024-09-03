use std::env;

use crate::database::postgres::credentials::DbCredentials;
use once_cell::sync::OnceCell;

pub static DB: OnceCell<DbCredentials> = OnceCell::new();
pub static JWT: OnceCell<JwtConfig> = OnceCell::new();

pub struct JwtConfig {
    pub secret: String,
}

pub fn setup() -> bool {
    dotenv::dotenv().ok();
    set_db() && set_jwt()
}

pub fn get_db() -> &'static DbCredentials {
    if DB.get().is_none() {
        set_db();
    }

    DB.get().expect("Database credentials not set")
}

pub fn set_db() -> bool {
    let db_creds = DbCredentials {
        database: env::var("DB_NAME").expect("DB_NAME not set in .env"),
        user: env::var("DB_USER").expect("DB_USER not set in .env"),
        password: env::var("DB_PASSWORD").expect("DB_PASSWORD not set in .env"),
        host: env::var("DB_HOST").expect("DB_HOST not set in .env"),
        port: env::var("DB_PORT")
            .expect("DB_PORT not set in .env")
            .parse::<u16>()
            .expect("Invalid DB_PORT value in .env"),
    };

    DB.set(db_creds).is_ok()
}

pub fn get_jwt() -> &'static JwtConfig {
    if JWT.get().is_none() {
        set_db();
    }

    JWT.get().expect("JWT config not set")
}

pub fn set_jwt() -> bool {
    let jwt_config = JwtConfig {
        secret: env::var("JWT_SECRET").expect("JWT_SECRET not set in .env"),
    };

    JWT.set(jwt_config).is_ok()
}
