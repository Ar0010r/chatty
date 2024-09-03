use std::{str, thread};

use crate::{
    email::gmail::{client::GmailClient, credentials::GmailCredentials},
    exception::model::Exception,
};

use super::{
    models::{Envelope, Message},
    traits::EmailClient,
};

pub struct Service<T: EmailClient> {
    client: T,
}

impl Service<GmailClient> {
    pub async fn build(email: String) -> Self {
        let creds = GmailCredentials::get(email.to_string());
        let auth = GmailClient::auth(creds).await;
        let client = GmailClient::new(auth, email.to_string()).await;

        Service::new(client)
    }
}
impl<T: EmailClient> Service<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }

    pub async fn send(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        attachments: &Vec<String>,
        reply_to: Option<&str>,
    ) -> Result<Message, Exception> {
        let msg = match reply_to {
            Some(r) => self.find(r).await,
            None => None,
        };

        let envelope = Envelope::make(to, subject, body, attachments, msg);

        self.dispatch(envelope).await
    }

    pub async fn dispatch(&self, message: Envelope) -> Result<Message, Exception> {
        self.client.send(message).await
    }

    pub async fn send_many(
        &self,
        messages: Vec<Envelope>,
        sleep: i16,
    ) -> Vec<Result<Message, Exception>> {
        let mut results = Vec::new();
        for m in messages {
            let result = self.client.send(m).await;
            results.push(result);

            thread::sleep(std::time::Duration::from_secs(sleep as u64));
        }

        results
    }

    pub async fn show(&self, msg_id: &str) -> Result<Message, Exception> {
        self.client.show(msg_id).await
    }

    pub async fn find(&self, msg_id: &str) -> Option<Message> {
        self.client.find(msg_id).await
    }

    pub async fn show_many(&self, msg_ids: Vec<&str>) -> Vec<Message> {
        let mut messages = Vec::new();
        for id in msg_ids {
            if let Ok(m) = self.show(id).await {
                messages.push(m);
            }
        }

        messages
    }

    pub async fn unread_list(&self) -> Vec<Message> {
        self.client.unread().await.unwrap_or_default()
    }

    pub async fn sent_list(&self) -> Vec<Message> {
        self.client.sent().await.unwrap_or_default()
    }

    pub async fn label(&self) -> Vec<Message> {
        self.client
            .label("2024-08-12-123")
            .await
            .unwrap_or_default()
    }

    pub async fn get_conversation(&self, email: &str) -> Vec<Message> {
        let from = self
            .client
            .list(format!("from:{}", email).as_str())
            .await
            .unwrap_or_default();
        let to = self
            .client
            .list(format!("to:{}", email).as_str())
            .await
            .unwrap_or_default();

        let mut messages: Vec<Message> = from.into_iter().chain(to.into_iter()).collect();

        messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

        messages
    }

    pub async fn find_last_recieved(&self, email: &str) -> Result<Option<Message>, Exception> {
        let from = self.client.list(format!("from:{}", email).as_str()).await?;
        let msg = from.iter().max_by(|a, b| a.timestamp.cmp(&b.timestamp));

        Ok(msg.cloned())
    }

    pub async fn show_last_recieved(&self, email: &str) -> Result<Message, Exception> {
        match self.find_last_recieved(email).await? {
            Some(msg) => Ok(msg),
            None => Err(Exception::not_found("No message found".to_string())),
        }
    }
}
