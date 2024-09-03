use domain::{
    dto::company::{CompanyBody, CompanyProps},
    entities::company,
    repositories::company::CompanyReader,
};
use sea_orm::{IntoActiveModel, Set};
use system::exception::model::Exception;

use super::BaseService;

pub type CompanyService = BaseService<company::Model, company::ActiveModel>;

impl CompanyService {
    pub async fn register(data: CompanyBody) -> Result<company::Model, Exception> {
        match CompanyReader::check_domain(&data.domain).await? {
            true => Err(Exception::exists("domain".to_string())),
            false => Ok(()),
        }?;

        CompanyService::create(data.to_model(), None).await
    }

    pub fn make(data: CompanyProps, original: company::Model) -> company::ActiveModel {
        let mut active = original.into_active_model();

        if data.name.is_some() {
            active.name = Set(data.name.unwrap());
        }

        if data.first_name.is_some() {
            active.first_name = Set(data.first_name.unwrap());
        }

        if data.last_name.is_some() {
            active.last_name = Set(data.last_name.unwrap());
        }

        if data.domain.is_some() {
            active.domain = Set(data.domain.unwrap());
        }

        if data.email.is_some() {
            active.email = Set(data.email.unwrap());
        }

        if data.manager_id.is_some() {
            active.manager_id = Set(data.manager_id.unwrap());
        }

        if data.status.is_some() {
            active.status = Set(data.status.unwrap());
        }

        if data.r#type.is_some() {
            active.r#type = Set(data.r#type.unwrap());
        }

        if data.credability.is_some() {
            active.credability = Set(data.credability.unwrap());
        }

        active
    }
}
