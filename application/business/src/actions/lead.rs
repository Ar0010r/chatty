use domain::{
    dto::{
        lead::{LeadBody, LeadFilter, LeadProps},
        manager::Manager,
    },
    entities::lead,
    repositories::lead::{LeadEraser, LeadReader},
    scope::lead::LeadScope,
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

use crate::service::lead::LeadService;

pub async fn create(data: LeadBody, resolver: &Manager) -> Result<lead::Model, Exception> {
    Validator::validate(&data)?;
    let data = data.set_owner(resolver);

    LeadService::register(data).await
}

pub async fn update(
    id: i64,
    data: LeadProps,
    resolver: &Manager,
) -> Result<lead::Model, Exception> {
    Validator::validate(&data)?;
    let model = show(id, resolver).await?;
    let active = LeadService::make(data, model);

    LeadService::modify(active, None).await
}

pub async fn show(id: i64, resolver: &Manager) -> Result<lead::Model, Exception> {
    let request = ListRequest {
        filters: LeadFilter::ids_only(vec![id]),
        ..Default::default()
    };

    match list(request, resolver).await?.data.pop() {
        Some(model) => Ok(model),
        None => Err(Exception::not_found(String::default())),
    }
}

pub async fn list(request: ListRequest<LeadFilter>, resolver: &Manager) -> ListResult<lead::Model> {
    let scope = LeadScope::for_select(resolver)?;
    LeadReader::scope(scope)
        .filter(request.filters)
        .search(&request.search_term.unwrap_or_default())
        .order_by::<lead::Column>(request.order_by)
        .paginate(request.page)
        .await
}

pub async fn delete(id: i64, resolver: &Manager) -> Result<bool, Exception> {
    let scope = LeadScope::for_delete(resolver)?;
    let delete = LeadEraser::by_id(id, Some(scope), None).await?;

    Ok(delete > 0)
}

pub async fn get_by_email(email: &str, resolver: &Manager) -> Result<lead::Model, Exception> {
    let emails = vec![email.to_string()];
    let scope = LeadScope::for_select(resolver)?;

    LeadReader::scope(scope).filter_emails(emails).show().await
}
