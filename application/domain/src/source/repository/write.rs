use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DatabaseTransaction, EntityTrait, InsertResult,
    IntoActiveModel,
};
use std::marker::PhantomData;
use system::{
    database::postgres::connection,
    exception::model::{Exception, ToException},
};

pub struct DataWriter<M> {
    _marker: PhantomData<M>,
}

impl<M> WritesData<M> for DataWriter<M>
where
    M: ActiveModelBehavior + Send,
    <<M as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<M>,
{
}

#[allow(async_fn_in_trait)]
pub trait WritesData<M>
where
    M: ActiveModelBehavior + Send,
    <<M as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<M>,
{
    async fn create(
        model: M,
        txn: Option<&DatabaseTransaction>,
    ) -> Result<<<M as ActiveModelTrait>::Entity as EntityTrait>::Model, Exception> {
        let r = match txn {
            Some(t) => model.insert(t).await,
            None => model.insert(connection::get()).await,
        };

        r.map_err(|e| e.to_exception())
    }

    async fn insert(
        models: Vec<M>,
        txn: Option<&DatabaseTransaction>,
    ) -> Result<InsertResult<M>, Exception> {
        let query = M::Entity::insert_many(models);
        let r = match txn {
            Some(t) => query.exec(t).await,
            None => query.exec(connection::get()).await,
        };

        r.map_err(|e| e.to_exception())
    }

    async fn update(
        model: M,
        txn: Option<&DatabaseTransaction>,
    ) -> Result<<<M as ActiveModelTrait>::Entity as EntityTrait>::Model, Exception> {
        let r = match txn {
            Some(t) => model.update(t).await,
            None => model.update(connection::get()).await,
        };

        r.map_err(|e| e.to_exception())
    }

    async fn save(model: M, txn: Option<&DatabaseTransaction>) -> Result<M, Exception> {
        let r = match txn {
            Some(t) => model.save(t).await,
            None => model.save(connection::get()).await,
        };

        r.map_err(|e| e.to_exception())
    }
}
