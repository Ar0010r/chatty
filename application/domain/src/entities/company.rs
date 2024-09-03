use chrono::NaiveDateTime;
use factory::Factory;
use fake::Dummy;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use crate::source::HasPrimaryColumn;

use super::{lead, manager};

#[derive(
    Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Default, Dummy,
)]
#[sea_orm(table_name = "companies")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub manager_id: i64,
    pub name: String,
    pub first_name: String,
    pub last_name: String,
    #[sea_orm(unique)]
    pub domain: String,
    #[sea_orm(unique)]
    pub email: String,
    pub status: Status,
    pub r#type: Type,
    pub credability: Credability,
    pub created_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Leads,
    Manager,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Leads => Entity::belongs_to(lead::Entity).into(),
            Self::Manager => Entity::belongs_to(manager::Entity).into(),
        }
    }
}

impl Related<super::lead::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Leads.def()
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
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, Dummy,
)]
#[sea_orm(rs_type = "i16", db_type = "TinyInteger")]
pub enum Status {
    #[default]
    #[sea_orm(num_value = 0)]
    None = 0,
    #[sea_orm(num_value = 1)]
    Active = 1,
    #[sea_orm(num_value = 2)]
    NotUsed = 2,
}

#[derive(
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, Dummy,
)]
#[sea_orm(rs_type = "i16", db_type = "TinyInteger")]
pub enum Credability {
    #[default]
    #[sea_orm(num_value = 0)]
    None = 0,
    #[sea_orm(num_value = 1)]
    Ok = 1,
    #[sea_orm(num_value = 2)]
    Bad = 2,
}

#[derive(
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default, Dummy,
)]
#[sea_orm(rs_type = "i16", db_type = "TinyInteger")]
pub enum Type {
    #[default]
    #[sea_orm(num_value = 0)]
    None = 0,
    #[sea_orm(num_value = 1)]
    Delivery = 1,
    #[sea_orm(num_value = 2)]
    Hr = 2,
}

impl Factory for Model {
    type Item = Self;

    fn create(&self) -> Self::Item {
        Self::default()
    }
}
