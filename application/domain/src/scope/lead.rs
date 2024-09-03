use sea_orm::{
    ColumnTrait, EntityTrait, JoinType, QueryFilter, QuerySelect, RelationTrait, Select,
};
use system::exception::model::Exception;

use crate::{
    dto::manager::Manager,
    entities::{company, lead},
    source::scope::{DataScope, DeleteScope, ViewScope},
};

pub type LeadScope = DataScope<lead::Entity>;

impl ViewScope<lead::Entity> for LeadScope {
    fn admin_scope(_mngr: &Manager) -> Result<Select<lead::Entity>, Exception> {
        Ok(lead::Entity::find())
    }

    fn hr_scope(mngr: &Manager) -> Result<Select<lead::Entity>, Exception> {
        Ok(lead::Entity::find().filter(lead::Column::HrId.eq(mngr.id)))
    }

    fn delivery_scope(mngr: &Manager) -> Result<Select<lead::Entity>, Exception> {
        Ok(<Self as ViewScope<lead::Entity>>::top_delivery_scope(mngr)?
            .join(JoinType::InnerJoin, lead::Relation::Company.def())
            .filter(company::Column::ManagerId.eq(mngr.id)))
    }

    fn top_delivery_scope(_mngr: &Manager) -> Result<Select<lead::Entity>, Exception> {
        Ok(lead::Entity::find().filter(lead::Column::Status.is_in(vec![
            lead::Status::Ready,
            lead::Status::Invited,
            lead::Status::Exported,
            lead::Status::Died,
        ])))
    }

    fn top_hr_scope(_mngr: &Manager) -> Result<Select<lead::Entity>, Exception> {
        Ok(lead::Entity::find())
    }
}

impl DeleteScope<lead::Entity> for LeadScope {
    fn admin_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<lead::Entity>, Exception> {
        Ok(lead::Entity::delete_many())
    }

    fn hr_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<lead::Entity>, Exception> {
        Err(Exception::unathorized(String::default()))
    }

    fn delivery_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<lead::Entity>, Exception> {
        Err(Exception::unathorized(String::default()))
    }

    fn top_delivery_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<lead::Entity>, Exception> {
        Err(Exception::unathorized(String::default()))
    }

    fn top_hr_scope(_mngr: &Manager) -> Result<sea_orm::DeleteMany<lead::Entity>, Exception> {
        Err(Exception::unathorized(String::default()))
    }
}
