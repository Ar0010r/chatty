use sea_orm::{
    ColumnTrait, DatabaseTransaction, DeleteMany, EntityTrait, FromQueryResult, PrimaryKeyTrait,
    QueryFilter,
};
use std::{convert, marker::PhantomData};
use system::{
    database::postgres::connection,
    exception::model::{Exception, ToException},
};

use crate::source::HasPrimaryColumn;

pub struct DataEraser<T> {
    _marker: PhantomData<T>,
}

impl<T> DeletesData<T> for DataEraser<T>
where
    T: EntityTrait + HasPrimaryColumn,
    <<T as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: convert::From<i64>,
    <T as EntityTrait>::Model: FromQueryResult + Sized + Send + Sync,
{
}

#[allow(async_fn_in_trait)]
pub trait DeletesData<T>
where
    T: EntityTrait + HasPrimaryColumn,
    <<T as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: convert::From<i64>,
    <T as EntityTrait>::Model: FromQueryResult + Sized + Send + Sync,
{
    async fn by_id(
        id: i64,
        scope: Option<DeleteMany<T>>,
        txn: Option<&DatabaseTransaction>,
    ) -> Result<u64, Exception> {
        let condition = T::primary_column().eq(id);
        let query = scope.unwrap_or(T::delete_many()).filter(condition);

        Self::by_query(query, txn).await
    }

    async fn by_ids(
        ids: Vec<i64>,
        scope: Option<DeleteMany<T>>,
        txn: Option<&DatabaseTransaction>,
    ) -> Result<u64, Exception> {
        let condition = T::primary_column().is_in(ids);
        let query = scope.unwrap_or(T::delete_many()).filter(condition);

        Self::by_query(query, txn).await
    }

    async fn by_query(
        query: DeleteMany<T>,
        txn: Option<&DatabaseTransaction>,
    ) -> Result<u64, Exception> {
        let result = match txn {
            Some(t) => query.exec(t).await,
            None => query.exec(connection::get()).await,
        };
        let result = result.map_err(|e| e.to_exception())?.rows_affected;

        Ok(result)
    }
}
