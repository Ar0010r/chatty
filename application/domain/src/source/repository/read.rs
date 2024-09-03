use sea_orm::{
    ColumnTrait, EntityTrait, FromQueryResult, Order, PaginatorTrait, PrimaryKeyTrait, QueryFilter,
    QueryOrder, Select,
};
use std::convert;
use system::{
    database::postgres::connection,
    exception::model::{Exception, ToException},
    models::{
        list::{List, OrderBy, PageRequest},
        shared::{ListResult, OptionResult, VecResult},
    },
};

use crate::source::HasPrimaryColumn;

pub struct DataReader<T>
where
    T: EntityTrait + HasPrimaryColumn,
    <<T as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: convert::From<i64>,
    <T as EntityTrait>::Model: FromQueryResult + Sized + Send + Sync,
{
    pub query: Select<T>,
}

impl<T> DataReader<T>
where
    T: EntityTrait + HasPrimaryColumn,
    <<T as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: convert::From<i64>,
    <T as EntityTrait>::Model: FromQueryResult + Sized + Send + Sync,
{
    pub fn new(query: Option<Select<T>>) -> Self {
        Self {
            query: query.unwrap_or(T::find()),
        }
    }

    pub fn scope(query: Select<T>) -> Self {
        Self { query }
    }

    pub fn order_by<C: ColumnTrait>(mut self, request: OrderBy) -> Self {
        let order = match request.direction.as_str() {
            "desc" => Order::Desc,
            _ => Order::Asc,
        };

        let column = match C::from_str(&request.column) {
            Ok(column) => Some(column),
            Err(_) => None,
        };

        if column.is_some() {
            self.query = self.query.order_by(column.unwrap(), order);
        }

        self
    }

    pub async fn paginate(self, request: PageRequest) -> ListResult<T::Model> {
        let per_page = match request.per_page {
            0 => 10,
            _ => request.per_page,
        };

        let p = self.query.paginate(connection::get(), per_page);

        let pages = match p.num_pages().await {
            Ok(pages) => Ok(pages),
            Err(error) => Err(error.to_exception()),
        }?;

        let result = match p.fetch_page(request.page).await {
            Ok(pages) => Ok(pages),
            Err(error) => Err(error.to_exception()),
        }?;

        Ok(List {
            data: result,
            count: pages,
        })
    }

    pub async fn all(self) -> Vec<T::Model> {
        self.query.all(connection::get()).await.unwrap()
    }

    pub async fn one(self) -> Result<Option<T::Model>, Exception> {
        self.query
            .one(connection::get())
            .await
            .map_err(|e| e.to_exception())
    }

    pub async fn show(self) -> Result<T::Model, Exception> {
        match self.one().await? {
            Some(result) => Ok(result),
            None => Err(Exception::not_found(String::default())),
        }
    }
}

impl<T> FindsData<T> for DataReader<T>
where
    T: EntityTrait + HasPrimaryColumn,
    <<T as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: convert::From<i64>,
    <T as EntityTrait>::Model: FromQueryResult + Sized + Send + Sync,
{
}

//#[async_trait]
#[allow(async_fn_in_trait)]
pub trait FindsData<T>
where
    T: EntityTrait + HasPrimaryColumn,
    <<T as EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType: convert::From<i64>,
    <T as EntityTrait>::Model: FromQueryResult + Sized + Send + Sync,
{
    async fn find(id: i64) -> OptionResult<T::Model> {
        match T::find_by_id(id).one(connection::get()).await {
            Ok(result) => Ok(result),
            Err(error) => Err(error.to_exception()),
        }
    }

    async fn find_many(ids: Vec<i64>) -> VecResult<T::Model> {
        let condition = T::primary_column().is_in(ids);

        match T::find().filter(condition).all(connection::get()).await {
            Ok(result) => Ok(result),
            Err(error) => Err(error.to_exception()),
        }
    }

    async fn find_or_fail(id: i64) -> Result<<T as EntityTrait>::Model, Exception> {
        let r = T::find_by_id(id).one(connection::get()).await;

        match r.map_err(|e| e.to_exception())? {
            Some(result) => Ok(result),
            None => Err(Exception::not_found(String::default())),
        }
    }
}
