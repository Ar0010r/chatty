use domain::{
    dto::manager::Manager, entities::lead_log::Action, repositories::company::CompanyReader,
    source::repository::read::FindsData,
};
use system::{
    email::shared::{
        models::{Message, SubjBody},
        service::Service,
        traits::EmailClient,
    },
    exception::model::Exception,
};

use crate::service::lead_log::LeadLogService;

use super::lead::get_by_email;

pub async fn get_conversation(email: &str, resolver: Manager) -> Result<Vec<Message>, Exception> {
    let lead = get_by_email(email, &resolver).await?;
    let hr_company_id = lead.hr_company_id.unwrap_or_default();
    let gmail_service = get_email_service(hr_company_id).await?;

    Ok(gmail_service.get_conversation(email).await)
}

pub async fn reply_lead(
    email: &str,
    reply_text: String,
    resolver: Manager,
) -> Result<Message, Exception> {
    let lead = get_by_email(email, &resolver).await?;
    let hr_company_id = lead.hr_company_id.unwrap_or_default();
    let gmail_service = get_email_service(hr_company_id).await?;
    let last_msg = gmail_service.show_last_recieved(email).await?;
    let reply = last_msg.reply(&reply_text, &Vec::new());
    let sent_msg = gmail_service.dispatch(reply).await?;
    LeadLogService::log_lead(&lead, Action::Reply).await?;

    Ok(sent_msg)
}

pub async fn write_to_lead(
    email: &str,
    msg: SubjBody,
    resolver: Manager,
) -> Result<Message, Exception> {
    let mut envelope = msg.to_envelope();
    envelope.to = email.to_string();
    let lead = get_by_email(email, &resolver).await?;
    let hr_company_id = lead.hr_company_id.unwrap_or_default();
    let gmail_service = get_email_service(hr_company_id).await?;
    let sent_msg = gmail_service.dispatch(envelope).await?;
    LeadLogService::log_lead(&lead, Action::Push).await?;

    Ok(sent_msg)
}

async fn get_email_service(hr_company_id: i64) -> Result<Service<impl EmailClient>, Exception> {
    let hr_company = CompanyReader::find_or_fail(hr_company_id).await?;

    Ok(Service::build(hr_company.email).await)
}
