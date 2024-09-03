use domain::{
    dto::manager::{ManagerBody, ManagerProps},
    entities::manager,
    repositories::manager::ManagerReader,
};
use sea_orm::{IntoActiveModel, Set};
use system::{exception::model::Exception, various::bcrypt};

use super::BaseService;

pub type ManagerService = BaseService<manager::Model, manager::ActiveModel>;

impl ManagerService {
    pub async fn register(data: ManagerBody) -> Result<manager::Model, Exception> {
        match ManagerReader::login_exists(&data.login).await? {
            true => Err(Exception::exists("login".to_string())),
            false => Ok(()),
        }?;

        let mut model = data.to_model();
        model.password = bcrypt::hash_once(&model.password);

        BaseService::create(model, None).await
    }

    pub async fn find_by_creds(
        login: &str,
        password: &str,
    ) -> Result<Option<manager::Model>, Exception> {
        let manager = ManagerReader::find_by_login(login).await?;

        if let Some(manager) = manager {
            return match bcrypt::verify(password, &manager.password) {
                true => Ok(Some(manager)),
                false => Err(Exception::invalid_creds()),
            };
        }

        Ok(None)
    }

    pub fn make(data: ManagerProps, original: manager::Model) -> manager::ActiveModel {
        let mut active = original.into_active_model();

        if data.login.is_some() {
            active.login = Set(data.login.unwrap());
        }

        if data.password.is_some() {
            active.password = Set(bcrypt::hash_once(&data.password.unwrap()));
        }

        if data.role.is_some() {
            active.role = Set(data.role.unwrap());
        }

        active
    }
}
