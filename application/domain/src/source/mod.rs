use sea_orm::ColumnTrait;
pub mod repository;
pub mod scope;

pub trait HasPrimaryColumn {
    type Column: ColumnTrait;

    fn primary_column() -> Self::Column;
}
