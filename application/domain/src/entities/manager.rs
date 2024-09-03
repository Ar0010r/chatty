use crate::source::HasPrimaryColumn;
use chrono::NaiveDateTime;
use fake::Dummy;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize, Dummy)]
#[sea_orm(table_name = "managers")]
pub struct Model {
    #[sea_orm(primary_key)]
    //#[serde(skip_deserializing)]
    pub id: i64,
    #[sea_orm(unique)]
    pub login: String,
    pub password: String,
    pub role: Role,
    pub created_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        has_many = "super::lead::Entity",
        from = "Column::Id",
        to = "super::lead::Column::HrId"
    )]
    Leads,
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
    EnumIter, DeriveActiveEnum, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Dummy,
)]
#[sea_orm(rs_type = "i16", db_type = "TinyInteger")]
#[derive(Default)]
pub enum Role {
    #[sea_orm(num_value = 0)]
    #[default]
    None,
    #[sea_orm(num_value = 1)]
    Hr,
    #[sea_orm(num_value = 2)]
    Delivery,
    #[sea_orm(num_value = 3)]
    TopHr,
    #[sea_orm(num_value = 4)]
    TopDelivery,
    #[sea_orm(num_value = 5)]
    Admin,
}

pub trait HasManagerId {
    fn manager_id(&self) -> i64;
}

impl HasManagerId for Model {
    fn manager_id(&self) -> i64 {
        self.id
    }
}

impl HasManagerId for ActiveModel {
    fn manager_id(&self) -> i64 {
        let value = match self.id.clone().into_value() {
            Some(id) => id,
            None => Value::Int(None),
        };

        match value {
            Value::BigInt(Some(id)) => id.to_owned(),
            _ => 0,
        }
    }
}
