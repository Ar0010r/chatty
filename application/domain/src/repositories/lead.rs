use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    prelude::Expr, sea_query::IntoCondition, ColumnTrait, ConnectionTrait, DbBackend, EntityTrait,
    JoinType, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, Statement,
};
use system::database::postgres::connection;
use system::exception::model::{Exception, ToException};

use crate::dto::lead::LeadFilter;
use crate::entities::{lead, lead_log};
use crate::source::repository::delete::DataEraser;
use crate::source::repository::read::DataReader;

pub type LeadReader = DataReader<lead::Entity>;
pub type LeadEraser = DataEraser<lead::Entity>;

impl LeadReader {
    pub fn filter(mut self, request: LeadFilter) -> Self {
        let ids = request.ids.unwrap_or_default();
        let statuses = request.statuses.unwrap_or_default();
        let first_name = request.first_name.unwrap_or_default();
        let last_name = request.last_name.unwrap_or_default();
        let emails = request.emails.unwrap_or_default();
        let email = request.email.unwrap_or_default();
        let hr_ids = request.hr_ids.unwrap_or_default();
        let company_ids = request.company_ids.unwrap_or_default();
        let hr_company_ids = request.hr_company_ids.unwrap_or_default();

        if !ids.is_empty() {
            self.query = self.query.filter(lead::Column::Id.is_in(ids));
        }

        if !statuses.is_empty() {
            self.query = self.query.filter(lead::Column::Status.is_in(statuses));
        }

        if !first_name.is_empty() {
            self.query = self
                .query
                .filter(lead::Column::FirstName.contains(first_name));
        }

        if !last_name.is_empty() {
            self.query = self
                .query
                .filter(lead::Column::LastName.contains(last_name));
        }

        if !hr_company_ids.is_empty() {
            self.query = self
                .query
                .filter(lead::Column::HrCompanyId.is_in(hr_company_ids));
        }

        if !company_ids.is_empty() {
            self.query = self
                .query
                .filter(lead::Column::CompanyId.is_in(company_ids));
        }

        if !hr_ids.is_empty() {
            self.query = self.query.filter(lead::Column::HrId.is_in(hr_ids));
        }

        if !email.is_empty() {
            let cond = "emails::TEXT ILIKE $1";
            let param = format!("%{}%", email);
            let expr = Expr::cust_with_values(cond, vec![param]);

            self.query = self.query.filter(expr.into_condition());
        }

        Self::filter_emails(self, emails)
    }

    pub fn search(mut self, term: &str) -> Self {
        if term.is_empty() {
            return self;
        }

        let term = format!("%{}%", term);

        let first_name = Expr::col(lead::Column::FirstName).ilike(&term);
        let last_name = Expr::col(lead::Column::LastName).ilike(&term);
        let email = Expr::cust_with_values("emails::TEXT ILIKE $1", vec![term]);
        let condition = first_name.or(last_name).or(email);

        self.query = self.query.filter(condition);

        self
    }

    pub fn filter_emails(mut self, emails: Vec<String>) -> Self {
        if !emails.is_empty() {
            let cond = "emails && $1::VARCHAR[]";
            let params = format!("{{{}}}", emails.join(","));
            let expr = Expr::cust_with_values(cond, vec![params]);

            self.query = self.query.filter(expr.into_condition());
        }

        self
    }
}

impl LeadReader {
    pub async fn check_email(email: &String) -> Result<bool, Exception> {
        let cond = "emails && $1::VARCHAR[]";
        let params = format!("{{{}}}", email);
        let expr = Expr::cust_with_values(cond, vec![params]);

        let result = lead::Entity::find()
            .filter(expr)
            .count(connection::get())
            .await
            .map_err(|err| err.to_exception())?;

        Ok(result > 0)
    }

    pub async fn sendout_list(filetrs: LeadFilter, vawe: i16) -> Vec<lead::Model> {
        let query = LeadReader::new(None).filter(filetrs).query;

        query
            .join(JoinType::LeftJoin, lead::Relation::Log.def())
            .group_by(lead::Column::Id)
            .having(lead_log::Column::Id.count().eq(vawe))
            .limit(50)
            // .having(Expr::count().eq(0))
            .all(connection::get())
            .await
            .unwrap()
    }

    pub async fn find_by_emails(emails: &[String]) -> Vec<lead::Model> {
        // making {email,email,email}
        let params = format!("{{{}}}", emails.join(","));
        let query = r#"SELECT * FROM leads WHERE emails && $1::VARCHAR[]"#;
        let stmt = Statement::from_sql_and_values(DbBackend::Postgres, query, [params.into()]);

        lead::Entity::find()
            .from_raw_sql(stmt)
            .all(connection::get())
            .await
            .unwrap()
    }

    pub async fn find_by_email(email: &str) -> Result<lead::Model, Exception> {
        let mut leads = Self::find_by_emails(&[email.to_string()]).await;

        match leads.pop() {
            Some(lead) => Ok(lead),
            None => Err(Exception::not_found("Lead not found".to_string())),
        }
    }

    pub async fn check_emails(emails: &[String]) -> Result<bool, Exception> {
        let params = format!("{{{}}}", emails.join(","));
        let query = r#"SELECT EXISTS ( 
            SELECT id FROM leads WHERE emails && $1::VARCHAR[] LIMIT 1
        ) "#;

        let stmt = Statement::from_sql_and_values(DbBackend::Postgres, query, [params.into()]);

        let r = connection::get()
            .query_one(stmt)
            .await
            .map_err(|err| err.to_exception())?
            .unwrap();

        r.try_get::<bool>("", "exists")
            .map_err(|err| err.to_exception())
    }
}
