use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Select};
use system::exception::model::Exception;

use crate::{
    dto::manager::Manager,
    entities::manager,
    source::scope::{DataScope, DeleteScope, ViewScope},
};

pub type ManagerScope = DataScope<manager::Entity>;

impl ViewScope<manager::Entity> for ManagerScope {
    fn admin_scope(_mngr: &Manager) -> Result<Select<manager::Entity>, Exception> {
        Ok(manager::Entity::find())
    }

    fn hr_scope(mngr: &Manager) -> Result<Select<manager::Entity>, Exception> {
        Ok(manager::Entity::find().filter(manager::Column::Id.eq(mngr.id)))
    }

    fn delivery_scope(mngr: &Manager) -> Result<Select<manager::Entity>, Exception> {
        Ok(manager::Entity::find().filter(manager::Column::Id.eq(mngr.id)))
    }

    fn top_delivery_scope(_mngr: &Manager) -> Result<Select<manager::Entity>, Exception> {
        Ok(manager::Entity::find().filter(manager::Column::Role.eq(manager::Role::Delivery)))
    }

    fn top_hr_scope(_mngr: &Manager) -> Result<Select<manager::Entity>, Exception> {
        Ok(manager::Entity::find().filter(manager::Column::Role.eq(manager::Role::Hr)))
    }
}

impl DeleteScope<manager::Entity> for ManagerScope {
    fn admin_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<manager::Entity>, Exception> {
        Ok(manager::Entity::delete_many())
    }

    fn hr_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<manager::Entity>, Exception> {
        Err(Exception::unathorized(String::default()))
    }

    fn delivery_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<manager::Entity>, Exception> {
        Err(Exception::unathorized(String::default()))
    }

    fn top_delivery_scope(
        _mngr: &Manager,
    ) -> Result<sea_orm::DeleteMany<manager::Entity>, Exception> {
        Err(Exception::unathorized(String::default()))
    }

    fn top_hr_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<manager::Entity>, Exception> {
        Err(Exception::unathorized(String::default()))
    }
}
