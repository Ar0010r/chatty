use crate::source::HasPrimaryColumn;
use chrono::NaiveDateTime;
use fake::Dummy;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use super::{company, lead_log, manager};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Dummy)]
#[sea_orm(table_name = "leads")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i64,
    pub hr_company_id: Option<i64>,
    pub company_id: Option<i64>,
    pub hr_id: Option<i64>,
    pub emails: Vec<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub status: Status,
    pub created_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Manager,
    Log,
    Company,
    HrCompany,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Manager => Entity::belongs_to(manager::Entity).into(),
            Self::Company => Entity::belongs_to(company::Entity).into(),
            Self::HrCompany => Entity::belongs_to(company::Entity).into(),
            Self::Log => Entity::has_many(lead_log::Entity).into(),
        }
    }
}

impl Related<super::manager::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Manager.def()
    }
}

impl Related<super::lead_log::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Log.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl HasPrimaryColumn for Entity {
    type Column = Column;
    fn primary_column() -> Self::Column {
        Column::Id
    }
}

#[derive(
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Dummy,
)]
#[sea_orm(rs_type = "i16", db_type = "TinyInteger")]
pub enum Status {
    #[sea_orm(num_value = 0)]
    None = 0,
    #[sea_orm(num_value = 1)]
    Active = 1,
    #[sea_orm(num_value = 2)]
    Ready = 2,
    #[sea_orm(num_value = 3)]
    Invited = 3,
    #[sea_orm(num_value = 4)]
    Exported = 4,
    #[sea_orm(num_value = 5)]
    Bad = 5,
    #[sea_orm(num_value = 6)]
    Died = 6,
}

#[derive(
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Dummy,
)]
#[sea_orm(rs_type = "i16", db_type = "TinyInteger")]
pub enum Label {
    #[sea_orm(num_value = 0)]
    None,
    #[sea_orm(num_value = 1)]
    Scam,
    #[sea_orm(num_value = 2)]
    NotUsa,
    #[sea_orm(num_value = 3)]
    Invited,
    #[sea_orm(num_value = 4)]
    Exported,
    #[sea_orm(num_value = 5)]
    Bad,
    #[sea_orm(num_value = 6)]
    Died,
}

impl Model {
    pub fn get_email(&self) -> String {
        match self.emails.first() {
            Some(email) => email.clone(),
            None => String::default(),
        }
    }
}
