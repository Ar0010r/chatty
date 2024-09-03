use serde::{Deserialize, Serialize};
use system::{
    exception::model::Exception,
    various::{auth::Authenticable, jwt::Cliams},
};

use crate::service::manager::ManagerService;

#[derive(Debug, Serialize, Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

impl Authenticable for Credentials {
    fn get_claims(&self) -> Cliams {
        Cliams {
            login: self.login.clone(),
        }
    }

    async fn validate(&self) -> Result<(), Exception> {
        match ManagerService::find_by_creds(&self.login, &self.password).await? {
            Some(_) => Ok(()),
            None => Err(Exception::invalid_creds()),
        }
    }
}
