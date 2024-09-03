use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Select};
use system::exception::model::Exception;

use crate::{
    dto::manager::Manager,
    entities::company,
    source::scope::{DataScope, DeleteScope, ViewScope},
};

pub type CompanyScope = DataScope<company::Entity>;

impl ViewScope<company::Entity> for CompanyScope {
    fn admin_scope(_mngr: &Manager) -> Result<Select<company::Entity>, Exception> {
        Ok(company::Entity::find())
    }

    fn hr_scope(mngr: &Manager) -> Result<Select<company::Entity>, Exception> {
        Ok(company::Entity::find().filter(
            company::Column::Type
                .eq(company::Type::Delivery)
                .or(company::Column::ManagerId.eq(mngr.id)),
        ))
    }

    fn delivery_scope(mngr: &Manager) -> Result<Select<company::Entity>, Exception> {
        Ok(company::Entity::find().filter(
            company::Column::ManagerId
                .eq(mngr.id)
                .and(company::Column::Type.eq(company::Type::Delivery)),
        ))
    }

    fn top_delivery_scope(_mngr: &Manager) -> Result<Select<company::Entity>, Exception> {
        Ok(company::Entity::find().filter(company::Column::Type.eq(company::Type::Delivery)))
    }

    fn top_hr_scope(_mngr: &Manager) -> Result<Select<company::Entity>, Exception> {
        Ok(company::Entity::find().filter(company::Column::Type.eq(company::Type::Hr)))
    }
}

impl DeleteScope<company::Entity> for CompanyScope {
    fn admin_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<company::Entity>, Exception> {
        Ok(company::Entity::delete_many())
    }

    fn hr_scope(mngr: &Manager) -> Result<sea_orm::DeleteMany<company::Entity>, Exception> {
        Ok(company::Entity::delete_many().filter(
            company::Column::Type
                .eq(company::Type::Delivery)
                .or(company::Column::ManagerId.eq(mngr.id)),
        ))
    }

    fn delivery_scope(mngr: &Manager) -> Result<sea_orm::DeleteMany<company::Entity>, Exception> {
        Ok(company::Entity::delete_many().filter(
            company::Column::ManagerId
                .eq(mngr.id)
                .and(company::Column::Type.eq(company::Type::Delivery)),
        ))
    }

    fn top_delivery_scope(
        _mngr: &Manager,
    ) -> Result<sea_orm::DeleteMany<company::Entity>, Exception> {
        Ok(
            company::Entity::delete_many()
                .filter(company::Column::Type.eq(company::Type::Delivery)),
        )
    }

    fn top_hr_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<company::Entity>, Exception> {
        Ok(company::Entity::delete_many().filter(company::Column::Type.eq(company::Type::Hr)))
    }
}
