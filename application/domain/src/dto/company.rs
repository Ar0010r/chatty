use chrono::{NaiveDateTime, Utc};
use fake::{faker::internet::en::SafeEmail, Dummy};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::entities::company::{self, Credability, Status, Type};

use super::manager::Manager;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CompanyFilter {
    pub ids: Option<Vec<i64>>,
    pub manager_ids: Option<Vec<i64>>,
    pub types: Option<Vec<Type>>,
    pub credabilities: Option<Vec<Credability>>,
    pub statuses: Option<Vec<Status>>,
    pub domain: Option<String>,
    pub email: Option<String>,
    pub created_from: Option<NaiveDateTime>,
    pub created_to: Option<NaiveDateTime>,
}

impl CompanyFilter {
    pub fn ids_only(ids: Vec<i64>) -> Self {
        Self {
            ids: Some(ids),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Validate, Default, Dummy)]
pub struct CompanyBody {
    #[validate(range(min = 1))]
    pub manager_id: i64,
    #[validate(length(min = 3))]
    pub first_name: String,
    #[validate(length(min = 3))]
    pub last_name: String,
    #[validate(length(min = 3))]
    pub name: String,
    #[validate(length(min = 3), url)]
    pub domain: String,
    #[validate(email)]
    #[dummy(faker = "SafeEmail()")]
    pub email: String,
    pub status: Status,
    pub r#type: Type,
    pub credability: Credability,
}

impl CompanyBody {
    pub fn to_model(self) -> company::Model {
        company::Model {
            id: i64::default(),
            name: self.name,
            manager_id: self.manager_id,
            first_name: self.first_name,
            last_name: self.last_name,
            domain: self.domain,
            email: self.email,
            status: self.status,
            r#type: self.r#type,
            credability: self.credability,
            created_at: Utc::now().naive_utc(),
        }
    }

    pub fn set_owner(mut self, manager: &Manager) -> Self {
        if manager.id > 0 {
            self.manager_id = manager.id;
        }

        self
    }
}

#[derive(Serialize, Deserialize, Debug, Validate, Default, Clone)]
pub struct CompanyProps {
    #[validate(length(min = 3))]
    pub name: Option<String>,
    #[validate(range(min = 1))]
    pub manager_id: Option<i64>,
    #[validate(length(min = 3))]
    pub first_name: Option<String>,
    #[validate(length(min = 3))]
    pub last_name: Option<String>,
    pub domain: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub status: Option<Status>,
    pub r#type: Option<Type>,
    pub credability: Option<Credability>,
}
