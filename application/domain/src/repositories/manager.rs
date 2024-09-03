use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use system::{
    database::postgres::connection,
    exception::model::{Exception, ToException},
};

use crate::{
    dto::manager::ManagerFilter,
    entities::manager,
    source::repository::{delete::DataEraser, read::DataReader, write::DataWriter},
};

pub type ManagerEraser = DataEraser<manager::Entity>;
pub type ManagerReader = DataReader<manager::Entity>;
pub type ManagerWriter = DataWriter<manager::Entity>;

impl ManagerReader {
    pub async fn find_by_login(login: &str) -> Result<Option<manager::Model>, Exception> {
        manager::Entity::find()
            .filter(manager::Column::Login.eq(login))
            .one(connection::get())
            .await
            .map_err(|err| Exception::error(err.to_string()))
    }

    pub async fn login_exists(login: &str) -> Result<bool, Exception> {
        let expr = manager::Column::Login.eq(login);
        let result = manager::Entity::find()
            .filter(expr)
            .count(connection::get())
            .await
            .map_err(|err| err.to_exception())?;

        Ok(result > 0)
    }
}

impl ManagerReader {
    pub fn filter(mut self, request: ManagerFilter) -> Self {
        if !request.login.is_empty() {
            self.query = self
                .query
                .filter(manager::Column::Login.contains(request.login));
        }

        if !request.roles.is_empty() {
            self.query = self
                .query
                .filter(manager::Column::Role.is_in(request.roles));
        }

        self
    }
}
