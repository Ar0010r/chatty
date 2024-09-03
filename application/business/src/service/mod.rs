use domain::source::{
    repository::write::{DataWriter, WritesData},
    HasPrimaryColumn,
};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, DatabaseTransaction, EntityTrait, IntoActiveModel,
};
use system::exception::model::Exception;

pub mod company;
pub mod lead;
pub mod lead_log;
pub mod manager;

pub struct BaseService<M, A> {
    _marker: std::marker::PhantomData<(M, A)>,
}

impl<M, A> BaseService<M, A>
where
    M: IntoActiveModel<A>,
    A: ActiveModelBehavior + Send,
    <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    <A as ActiveModelTrait>::Entity: EntityTrait + HasPrimaryColumn,
    <<A as ActiveModelTrait>::Entity as EntityTrait>::Column:
        From<<<A as ActiveModelTrait>::Entity as HasPrimaryColumn>::Column>,
{
    pub async fn create(
        model: M,
        txn: Option<&DatabaseTransaction>,
    ) -> Result<<<A as ActiveModelTrait>::Entity as EntityTrait>::Model, Exception> {
        let mut active = model.into_active_model();
        active.not_set(A::Entity::primary_column().into());

        match DataWriter::create(active, txn).await {
            Ok(res) => Ok(res),
            Err(err) => Err(Exception::error(err.to_string())),
        }
    }

    pub async fn modify(
        model: A,
        txn: Option<&DatabaseTransaction>,
    ) -> Result<<<A as ActiveModelTrait>::Entity as EntityTrait>::Model, Exception> {
        match DataWriter::update(model, txn).await {
            Ok(res) => Ok(res),
            Err(err) => Err(Exception::error(err.to_string())),
        }
    }

    pub async fn insert(
        models: Vec<M>,
        txn: Option<&DatabaseTransaction>,
    ) -> Result<(), Exception> {
        let active = models
            .into_iter()
            .map(|model| {
                let mut m = model.into_active_model();
                m.not_set(A::Entity::primary_column().into());
                m
            })
            .collect();

        match DataWriter::insert(active, txn).await {
            Ok(_) => Ok(()),
            Err(err) => Err(Exception::error(err.to_string())),
        }
    }

    // pub async fn save(model: M, txn: Option<&DatabaseTransaction>) -> Result<<<A as ActiveModelTrait>::Entity as EntityTrait>::Model, Exception> {
    //     let active = model.into_active_model();
    //     let result = WriteRepository::save(active, txn).await;

    //     match result {
    //         Ok(model) => Ok(model),
    //         Err(msg) => Err(Exception::error(err.to_string()))
    //     }
    // }
}
