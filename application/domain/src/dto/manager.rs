use chrono::{NaiveDateTime, Utc};
use fake::Dummy;
use serde::{Deserialize, Serialize};
use system::models::list::{OrderBy, PageRequest};
use validator::Validate;

use crate::entities::manager::{Model, Role};

#[derive(Serialize, Deserialize, Debug, Validate, Dummy)]
pub struct ManagerBody {
    #[validate(length(min = 3))]
    pub login: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub role: i8,
}

impl ManagerBody {
    pub fn to_model(&self) -> Model {
        Model {
            id: i64::default(),
            login: self.login.clone(),
            password: self.password.clone(),
            role: Role::Hr,
            created_at: Utc::now().naive_utc(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Manager {
    pub id: i64,
    pub login: String,
    pub role: Role,
    pub created_at: NaiveDateTime,
}

impl Manager {
    pub fn from_model(model: Model) -> Self {
        Self {
            id: model.id,
            login: model.login,
            role: model.role,
            created_at: model.created_at,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ManagerRequest {
    pub page: PageRequest,
    pub order: OrderBy,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ManagerFilter {
    pub ids: Option<Vec<i64>>,
    pub login: String,
    pub roles: Vec<Role>,
}

impl ManagerFilter {
    pub fn ids_only(ids: Vec<i64>) -> Self {
        Self {
            ids: Some(ids),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Validate)]
pub struct ManagerProps {
    #[validate(length(min = 3))]
    pub login: Option<String>,
    #[validate(length(min = 6))]
    pub password: Option<String>,
    pub role: Option<Role>,
}
