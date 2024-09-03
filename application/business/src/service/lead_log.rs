use domain::entities::{
    lead,
    lead_log::{self, Action},
};
use system::exception::model::Exception;

use super::BaseService;

pub type LeadLogService = BaseService<lead_log::Model, lead_log::ActiveModel>;

impl LeadLogService {
    pub async fn log_lead(
        lead: &lead::Model,
        action: Action,
    ) -> Result<lead_log::Model, Exception> {
        let log = lead_log::Model {
            id: 0,
            lead_id: lead.id,
            action,
            created_at: chrono::Utc::now().naive_utc(),
        };

        BaseService::create(log, None).await
    }
}
