use chrono::Utc;
use fake::Dummy;
use serde::{Deserialize, Serialize};
use system::various::validation::validate_emails;
use validator::Validate;

use crate::entities::lead::{self, Status};

use super::manager::Manager;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LeadFilter {
    pub ids: Option<Vec<i64>>,
    pub statuses: Option<Vec<Status>>,
    pub hr_ids: Option<Vec<i16>>,
    pub company_ids: Option<Vec<String>>,
    pub hr_company_ids: Option<Vec<String>>,
    pub emails: Option<Vec<String>>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
}

impl LeadFilter {
    pub fn ids_only(ids: Vec<i64>) -> Self {
        Self {
            ids: Some(ids),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, Dummy)]
pub struct LeadBody {
    #[validate(range(min = 1))]
    pub hr_company_id: Option<i64>,
    #[validate(range(min = 1))]
    pub company_id: Option<i64>,
    #[validate(range(min = 1))]
    pub hr_id: Option<i64>,
    #[validate(length(min = 1), custom(function = "validate_emails"))]
    pub emails: Vec<String>,
    #[validate(length(min = 1))]
    pub first_name: Option<String>,
    #[validate(length(min = 1))]
    pub last_name: Option<String>,
    pub status: Status,
}

impl LeadBody {
    pub fn to_model(self) -> lead::Model {
        lead::Model {
            id: Default::default(),
            hr_company_id: self.hr_company_id,
            company_id: self.company_id,
            hr_id: self.hr_id,
            emails: self.emails,
            first_name: self.first_name,
            last_name: self.last_name,
            status: self.status,
            created_at: Utc::now().naive_utc(),
        }
    }

    pub fn set_owner(mut self, manager: &Manager) -> Self {
        if manager.id > 0 {
            self.hr_id = Some(manager.id);
        }

        self
    }
}

#[derive(Debug, Serialize, Deserialize, Validate, Default, Clone)]
pub struct LeadProps {
    #[validate(range(min = 1))]
    pub hr_company_id: Option<Option<i64>>,
    #[validate(range(min = 1))]
    pub company_id: Option<Option<i64>>,
    #[validate(range(min = 1))]
    pub hr_id: Option<Option<i64>>,
    #[validate(length(min = 1), custom(function = "validate_emails"))]
    pub emails: Option<Vec<String>>,
    #[validate(length(min = 1))]
    pub first_name: Option<Option<String>>,
    #[validate(length(min = 1))]
    pub last_name: Option<Option<String>>,
    pub status: Option<Status>,
}
