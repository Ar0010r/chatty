use chrono::NaiveDateTime;
use fake::Dummy;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

use crate::source::HasPrimaryColumn;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Dummy)]
#[sea_orm(table_name = "lead_log")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub lead_id: i64,
    pub action: Action,
    pub created_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::lead::Entity",
        from = "Column::LeadId",
        to = "super::lead::Column::Id"
    )]
    Lead,
}

impl Related<super::lead::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Lead.def()
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
pub enum Action {
    #[sea_orm(num_value = 0)]
    None,
    #[sea_orm(num_value = 1)]
    Push,
    #[sea_orm(num_value = 2)]
    Spam,
    #[sea_orm(num_value = 3)]
    Reply,
}
