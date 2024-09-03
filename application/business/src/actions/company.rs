use crate::service::company::CompanyService;
use domain::{
    dto::{
        company::{CompanyBody, CompanyFilter, CompanyProps},
        manager::Manager,
    },
    entities::company,
    repositories::company::{CompanyEraser, CompanyReader},
    scope::company::CompanyScope,
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

pub async fn list(
    request: ListRequest<CompanyFilter>,
    resolver: &Manager,
) -> ListResult<company::Model> {
    let scope = CompanyScope::for_select(resolver)?;
    CompanyReader::scope(scope)
        .filter(request.filters)
        .order_by::<company::Column>(request.order_by)
        .paginate(request.page)
        .await
}

pub async fn create(data: CompanyBody, resolver: &Manager) -> Result<company::Model, Exception> {
    Validator::validate(&data)?;
    let data = data.set_owner(resolver);
    let r = CompanyService::register(data).await?;

    Ok(r)
}

pub async fn show(id: i64, resolver: &Manager) -> Result<company::Model, Exception> {
    let request = ListRequest {
        filters: CompanyFilter::ids_only(vec![id]),
        ..Default::default()
    };

    match list(request, resolver).await?.data.pop() {
        Some(model) => Ok(model),
        None => Err(Exception::not_found(String::default())),
    }
}

pub async fn update(
    id: i64,
    data: CompanyProps,
    resolver: &Manager,
) -> Result<company::Model, Exception> {
    Validator::validate(&data)?;
    let model = show(id, resolver).await?;
    let active = CompanyService::make(data, model);

    CompanyService::modify(active, None).await
}

pub async fn delete(id: i64, resolver: &Manager) -> Result<bool, Exception> {
    let scope = CompanyScope::for_delete(resolver)?;
    let delete = CompanyEraser::by_id(id, Some(scope), None).await?;

    Ok(delete > 0)
}

pub async fn delete_many(ids: Vec<i64>, resolver: &Manager) -> Result<bool, Exception> {
    let scope = CompanyScope::for_delete(resolver)?;
    let delete = CompanyEraser::by_ids(ids, Some(scope), None).await?;

    Ok(delete > 0)
}
