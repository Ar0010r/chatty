use crate::{dto::manager::Manager, entities::manager::Role};
use sea_orm::{DeleteMany, EntityTrait, Select};
use system::exception::model::Exception;
pub struct DataScope<T: EntityTrait> {
    _phantom: std::marker::PhantomData<T>,
}

pub trait ViewScope<T: EntityTrait> {
    fn for_select(mngr: &Manager) -> Result<Select<T>, Exception> {
        match mngr.role {
            Role::Admin => Self::admin_scope(mngr),
            Role::Hr => Self::hr_scope(mngr),
            Role::Delivery => Self::delivery_scope(mngr),
            Role::TopDelivery => Self::top_delivery_scope(mngr),
            Role::TopHr => Self::top_hr_scope(mngr),
            _ => Err(Exception::unathorized(String::default())),
        }
    }

    fn admin_scope(mngr: &Manager) -> Result<Select<T>, Exception>;
    fn hr_scope(mngr: &Manager) -> Result<Select<T>, Exception>;
    fn delivery_scope(mngr: &Manager) -> Result<Select<T>, Exception>;
    fn top_delivery_scope(mngr: &Manager) -> Result<Select<T>, Exception>;
    fn top_hr_scope(mngr: &Manager) -> Result<Select<T>, Exception>;
}

pub trait DeleteScope<T: EntityTrait> {
    fn for_delete(mngr: &Manager) -> Result<DeleteMany<T>, Exception> {
        match mngr.role {
            Role::Admin => Self::admin_scope(mngr),
            Role::Hr => Self::hr_scope(mngr),
            Role::Delivery => Self::delivery_scope(mngr),
            Role::TopDelivery => Self::top_delivery_scope(mngr),
            Role::TopHr => Self::top_hr_scope(mngr),
            _ => Err(Exception::unathorized(String::default())),
        }
    }

    fn admin_scope(mngr: &Manager) -> Result<DeleteMany<T>, Exception>;
    fn hr_scope(mngr: &Manager) -> Result<DeleteMany<T>, Exception>;
    fn delivery_scope(mngr: &Manager) -> Result<DeleteMany<T>, Exception>;
    fn top_delivery_scope(mngr: &Manager) -> Result<DeleteMany<T>, Exception>;
    fn top_hr_scope(mngr: &Manager) -> Result<DeleteMany<T>, Exception>;
}
