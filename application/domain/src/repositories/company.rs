use chrono::NaiveDateTime;
use sea_orm::{ColumnTrait, ConnectionTrait, DbBackend, QueryFilter, Statement};
use system::{
    database::postgres::connection,
    exception::model::{Exception, ToException},
};

use crate::{
    dto::company::CompanyFilter,
    entities::company,
    source::repository::{delete::DataEraser, read::DataReader, write::DataWriter},
};

//pub type ReadCompanyRepository = ReadRepository<company::Entity>;

pub type WriteCompanyRepository = DataWriter<company::Entity>;

pub type CompanyReader = DataReader<company::Entity>;

pub type CompanyEraser = DataEraser<company::Entity>;

impl CompanyReader {
    pub fn filter(mut self, request: CompanyFilter) -> Self {
        let ids = request.ids.unwrap_or_default();
        let types = request.types.unwrap_or_default();
        let credabilities = request.credabilities.unwrap_or_default();
        let statuses = request.statuses.unwrap_or_default();
        let manager_ids = request.manager_ids.unwrap_or_default();
        let created_from = request.created_from.unwrap_or_default();
        let created_to = request.created_to.unwrap_or_default();
        let domain = request.domain.unwrap_or_default();
        let email = request.email.unwrap_or_default();

        if !ids.is_empty() {
            self.query = self.query.filter(company::Column::Id.is_in(ids));
        }

        if !types.is_empty() {
            self.query = self.query.filter(company::Column::Type.is_in(types));
        }

        if !credabilities.is_empty() {
            self.query = self
                .query
                .filter(company::Column::Credability.is_in(credabilities));
        }

        if !statuses.is_empty() {
            self.query = self.query.filter(company::Column::Status.is_in(statuses));
        }

        if !manager_ids.is_empty() {
            self.query = self
                .query
                .filter(company::Column::ManagerId.is_in(manager_ids));
        }

        if created_from != NaiveDateTime::default() {
            self.query = self
                .query
                .filter(company::Column::CreatedAt.gte(created_from));
        }

        if created_to != NaiveDateTime::default() {
            self.query = self
                .query
                .filter(company::Column::CreatedAt.lte(created_to));
        }

        if !domain.is_empty() {
            self.query = self.query.filter(company::Column::Domain.like(domain));
        }

        if !email.is_empty() {
            self.query = self.query.filter(company::Column::Email.like(email));
        }

        self
    }

    pub async fn check_domain(domain: &String) -> Result<bool, Exception> {
        let query = r#"SELECT EXISTS ( 
        SELECT id FROM companies WHERE domain = $1 LIMIT 1
    ) "#;

        let stmt = Statement::from_sql_and_values(DbBackend::Postgres, query, [domain.into()]);

        let r = connection::get()
            .query_one(stmt)
            .await
            .map_err(|err| err.to_exception())?
            .unwrap();

        r.try_get::<bool>("", "exists")
            .map_err(|err| err.to_exception())
    }
}
