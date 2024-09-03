use std::future::Future;

use crate::exception::model::Exception;

use super::models::{Envelope, Message};

pub trait MessageConvertible<T> {
    fn to_message(&self) -> Message;
    fn from_message(mesage: Message) -> T;
}

pub trait MessageParts {
    fn get_id(&self) -> String;
    fn get_from(&self) -> String;
    fn get_to(&self) -> String;
    fn get_subject(&self) -> String;
    fn get_body(&self) -> String;
    fn get_reply_to(&self) -> String;
    fn get_thread_id(&self) -> String;
    fn get_timestamp(&self) -> i64;
}

pub trait EmailClient {
    fn send(
        &self,
        envelope: Envelope,
    ) -> impl Future<Output = Result<Message, Exception>> + std::marker::Send;
    fn show_many(&self, msg_ids: Vec<&str>) -> impl Future<Output = Vec<Message>> + Send;
    fn find(&self, msg_id: &str) -> impl Future<Output = Option<Message>> + Send;
    fn show(&self, msg_id: &str) -> impl Future<Output = Result<Message, Exception>> + Send;
    fn label(&self, label: &str) -> impl Future<Output = Result<Vec<Message>, Exception>> + Send;
    fn list(
        &self,
        q: &str,
    ) -> impl std::future::Future<Output = Result<Vec<Message>, Exception>> + Send;
    fn sent(&self) -> impl std::future::Future<Output = Result<Vec<Message>, Exception>> + Send;
    fn unread(&self) -> impl std::future::Future<Output = Result<Vec<Message>, Exception>> + Send;
}
