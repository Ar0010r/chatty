use domain::{
    dto::lead::{LeadBody, LeadProps},
    entities::lead,
    repositories::lead::LeadReader,
};
use sea_orm::{IntoActiveModel, Set};
use system::exception::model::Exception;

use super::BaseService;

pub type LeadService = BaseService<lead::Model, lead::ActiveModel>;

impl LeadService {
    pub async fn register(data: LeadBody) -> Result<lead::Model, Exception> {
        match LeadReader::check_emails(&data.emails).await? {
            true => Err(Exception::exists("email".to_string())),
            false => Ok(()),
        }?;

        BaseService::create(data.to_model(), None).await
    }

    pub fn make(data: LeadProps, original: lead::Model) -> lead::ActiveModel {
        let mut active = original.into_active_model();

        if data.hr_company_id.is_some() {
            active.hr_company_id = Set(data.hr_company_id.unwrap());
        }

        if data.company_id.is_some() {
            active.company_id = Set(data.company_id.unwrap());
        }

        if data.hr_id.is_some() {
            active.hr_id = Set(data.hr_id.unwrap());
        }

        if data.first_name.is_some() {
            active.first_name = Set(data.first_name.unwrap());
        }

        if data.last_name.is_some() {
            active.last_name = Set(data.last_name.unwrap());
        }

        if data.emails.is_some() {
            active.emails = Set(data.emails.unwrap());
        }

        if data.status.is_some() {
            active.status = Set(data.status.unwrap());
        }

        active
    }
}
