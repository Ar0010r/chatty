use domain::{
    dto::manager::{Manager, ManagerBody, ManagerFilter, ManagerProps},
    entities::manager,
    repositories::manager::{ManagerEraser, ManagerReader},
    scope::manager::ManagerScope,
    source::{
        repository::delete::DeletesData,
        scope::{DeleteScope, ViewScope},
    },
};
use system::{
    exception::model::Exception,
    models::{list::ListRequest, shared::ListResult},
    various::validation::Validator,
};

use crate::{service::manager::ManagerService, various::policies::Policy};

pub async fn list(
    request: ListRequest<ManagerFilter>,
    resolver: &Manager,
) -> ListResult<manager::Model> {
    let scope = ManagerScope::for_select(resolver)?;
    ManagerReader::scope(scope)
        .filter(request.filters)
        .order_by::<manager::Column>(request.order_by)
        .paginate(request.page)
        .await
}

pub async fn create(data: ManagerBody, resolver: &Manager) -> Result<manager::Model, Exception> {
    Policy::admin_only(resolver)?;
    Validator::validate(&data)?;

    ManagerService::register(data).await
}

pub async fn show(id: i64, resolver: &Manager) -> Result<Option<manager::Model>, Exception> {
    let request = ListRequest {
        filters: ManagerFilter::ids_only(vec![id]),
        ..Default::default()
    };

    Ok(list(request, resolver).await?.data.pop())
}

pub async fn update(
    id: i64,
    data: ManagerProps,
    resolver: &Manager,
) -> Result<manager::Model, Exception> {
    Validator::validate(&data)?;
    let model = match show(id, resolver).await? {
        Some(model) => Ok(ManagerService::make(data, model)),
        None => Err(Exception::not_found(String::default())),
    }?;

    Policy::owner_only(resolver, &model)?;
    ManagerService::modify(model, None).await
}

pub async fn delete(id: i64, resolver: &Manager) -> Result<bool, Exception> {
    Policy::admin_only(resolver)?;
    let scope = ManagerScope::for_delete(resolver)?;
    let delete = ManagerEraser::by_id(id, Some(scope), None).await?;

    Ok(delete > 0)
}
