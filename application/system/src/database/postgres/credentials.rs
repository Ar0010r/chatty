use crate::config;

#[derive(Debug)]
pub struct DbCredentials {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String,
}

impl DbCredentials {
    pub fn get_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}?currentSchema=public",
            self.user, self.password, self.host, self.port, self.database
        )
    }
}

pub fn get() -> &'static DbCredentials {
    config::get_db()
}
