use axum::{
    async_trait,
    extract::{rejection::FormRejection, FromRequest, Request},
    Form, Json,
};
use domain::{
    dto::manager::Manager,
    entities::manager::{HasManagerId, Role},
};
use serde::de::DeserializeOwned;
use system::exception::model::Exception;

pub struct Policy;

impl Policy {
    pub fn admin_only(manager: &Manager) -> Result<(), Exception> {
        match manager.role {
            Role::Admin => Ok(()),
            _ => Err(Exception::unathorized(String::default())),
        }
    }

    pub fn hr_only(manager: &Manager) -> Result<(), Exception> {
        match manager.role {
            Role::Hr => Ok(()),
            _ => Self::top_hr_only(manager),
        }
    }

    pub fn top_hr_only(manager: &Manager) -> Result<(), Exception> {
        match manager.role {
            Role::TopHr => Ok(()),
            _ => Self::admin_only(manager),
        }
    }

    pub fn delivery_only(manager: &Manager) -> Result<(), Exception> {
        match manager.role {
            Role::Delivery => Ok(()),
            _ => Self::top_delivery_only(manager),
        }
    }

    pub fn top_delivery_only(manager: &Manager) -> Result<(), Exception> {
        match manager.role {
            Role::TopDelivery => Ok(()),
            _ => Self::admin_only(manager),
        }
    }

    pub fn owner_only(manager: &Manager, resourse: &impl HasManagerId) -> Result<(), Exception> {
        match manager.id == resourse.manager_id() {
            true => Ok(()),
            false => Policy::admin_only(manager),
        }
    }

    pub fn hr_owner(manager: &Manager, resourse: &impl HasManagerId) -> Result<(), Exception> {
        match manager.id == resourse.manager_id() {
            true => Ok(()),
            false => Policy::top_hr_only(manager),
        }
    }

    pub fn delivery_owner(
        manager: &Manager,
        resourse: &impl HasManagerId,
    ) -> Result<(), Exception> {
        match manager.id == resourse.manager_id() {
            true => Ok(()),
            false => Policy::top_delivery_only(manager),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AuthorizeAdmin<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for AuthorizeAdmin<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = Exception;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let ext = &req.extensions();

        let manager = match ext.get::<Manager>() {
            Some(user) => Ok(user.clone()),
            None => Err(Exception::unathorized(String::default())),
        }?;

        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| Exception::bad_request(e.to_string()))?;

        match Policy::admin_only(&manager) {
            Ok(_) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AuthorizeOwner<T: HasManagerId>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for AuthorizeOwner<T>
where
    T: DeserializeOwned + HasManagerId,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = Exception;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let ext = &req.extensions();

        let manager = match ext.get::<Manager>() {
            Some(user) => Ok(user.clone()),
            None => Err(Exception::unathorized(String::default())),
        }?;

        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| Exception::bad_request(e.to_string()))?;

        match Policy::owner_only(&manager, &value) {
            Ok(_) => Ok(Self(value)),
            Err(e) => Err(e),
        }
    }
}
