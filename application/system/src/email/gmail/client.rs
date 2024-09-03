use google_gmail1::oauth2::hyper_rustls::HttpsConnector as HyperHttpsConnector;
use google_gmail1::{
    api::{Message as GmailMessage, Scope},
    hyper::{client::HttpConnector, Client as HyperClient},
    hyper_rustls::{HttpsConnector, HttpsConnectorBuilder},
    oauth2::{authenticator::Authenticator, ServiceAccountAuthenticator},
    Gmail,
};
use std::io::Cursor;

use crate::{
    email::shared::{
        models::{Envelope, Message},
        traits::{EmailClient, MessageConvertible},
    },
    exception::model::{Exception, ToException},
};

use lettre::Message as LettreMessage;

use super::credentials::GmailCredentials;

pub struct GmailClient {
    pub hub: Gmail<HttpsConnector<HttpConnector>>,
    pub user_id: String,
}

impl GmailClient {
    pub async fn new(
        auth: Authenticator<HyperHttpsConnector<HttpConnector>>,
        user_id: String,
    ) -> GmailClient {
        let connector = HttpsConnectorBuilder::new()
            .with_native_roots()
            .unwrap()
            .https_only()
            .enable_http1()
            .build();
        let hub = Gmail::new(HyperClient::builder().build(connector), auth);

        Self { hub, user_id }
    }

    pub async fn auth(
        creds: GmailCredentials,
    ) -> Authenticator<HyperHttpsConnector<HttpConnector>> {
        ServiceAccountAuthenticator::builder(creds.key)
            .persist_tokens_to_disk(creds.token_location.to_str().unwrap())
            .subject(creds.email)
            .build()
            .await
            .unwrap()
    }
}

impl EmailClient for GmailClient {
    async fn unread(&self) -> Result<Vec<Message>, Exception> {
        let list = match self
            .hub
            .users()
            .messages_list(&self.user_id)
            .q("is:unread")
            .doit()
            .await
        {
            Ok((_, response)) => Ok(response),
            Err(e) => Err(e.to_exception()),
        }?;

        let messages = list.messages.unwrap_or_default();
        let ids = messages
            .iter()
            .map(|m| m.id.as_ref().unwrap().as_str())
            .collect();

        Ok(self.show_many(ids).await)
    }

    async fn sent(&self) -> Result<Vec<Message>, Exception> {
        let list = match self
            .hub
            .users()
            .messages_list(&self.user_id)
            .q("in:sent")
            .max_results(15)
            .doit()
            .await
        {
            Ok((_, response)) => Ok(response),
            Err(e) => Err(e.to_exception()),
        }?;

        let messages = list.messages.unwrap_or_default();
        let ids = messages
            .iter()
            .map(|m| m.id.as_ref().unwrap().as_str())
            .collect();

        Ok(self.show_many(ids).await)
    }

    async fn list(&self, q: &str) -> Result<Vec<Message>, Exception> {
        let list = match self
            .hub
            .users()
            .messages_list(&self.user_id)
            .q(q)
            .doit()
            .await
        {
            Ok((_, response)) => Ok(response),
            Err(e) => Err(e.to_exception()),
        }?;

        let messages = list.messages.unwrap_or_default();
        let ids = messages
            .iter()
            .map(|m| m.id.as_ref().unwrap().as_str())
            .collect();

        Ok(self.show_many(ids).await)
    }

    async fn label(&self, label: &str) -> Result<Vec<Message>, Exception> {
        let list = match self
            .hub
            .users()
            .messages_list(&self.user_id)
            .add_label_ids(label)
            .doit()
            .await
        {
            Ok((_, response)) => Ok(response),
            Err(e) => Err(e.to_exception()),
        }?;

        let messages = list.messages.unwrap_or_default();
        let ids = messages
            .iter()
            .map(|m| m.id.as_ref().unwrap().as_str())
            .collect();

        Ok(self.show_many(ids).await)
    }

    async fn show(&self, msg_id: &str) -> Result<Message, Exception> {
        if msg_id.is_empty() {
            return Err(Exception::error("Message ID is required".to_string()));
        };

        // might be needed to add .add_scope(Scope::Metadata).add_scope(Scope::Modify).add_scope(Scope::Gmai)
        let result = self
            .hub
            .users()
            .messages_get(&self.user_id, msg_id)
            .clear_scopes()
            .add_scope(Scope::Readonly)
            .doit()
            .await;

        match result {
            Ok((_, response)) => Ok(response.to_message()),
            Err(e) => Err(e.to_exception()),
        }
    }

    async fn find(&self, msg_id: &str) -> Option<Message> {
        match self.show(msg_id).await {
            Ok(m) => Some(m),
            Err(_) => None,
        }
    }

    async fn show_many(&self, msg_ids: Vec<&str>) -> Vec<Message> {
        let mut messages = Vec::new();
        for id in msg_ids {
            if let Ok(m) = self.show(id).await {
                messages.push(m);
            }
        }

        messages
    }

    async fn send(&self, envelope: Envelope) -> Result<Message, Exception> {
        let mut message = envelope.to_message();
        message.from = self.user_id.clone();

        let email = LettreMessage::from_message(message);

        let (_, msg) = self
            .hub
            .users()
            .messages_send(GmailMessage::default(), self.user_id.as_str())
            .upload_resumable(
                Cursor::new(email.formatted()),
                "message/rfc822".parse().expect("mime rfc822 is valid"),
            )
            .await
            .map_err(|e| e.to_exception())?;

        Ok(msg.to_message())
    }
}
