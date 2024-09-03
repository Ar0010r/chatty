use serde::{Deserialize, Serialize};
use std::u8;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
    pub reply_to: String,
    pub id: String,
    pub thread_id: String,
    pub attachments: Vec<String>,
    pub timestamp: i64,
}

impl Message {
    pub fn make(
        to: &str,
        subject: &str,
        body: &str,
        from: &str,
        attachments: &Vec<String>,
        reply_to: Option<Message>,
    ) -> Message {
        let reply_subject = match &reply_to {
            Some(msg) => msg.subject.to_owned(),
            None => String::default().as_str().to_string(),
        };

        let reply_id = match &reply_to {
            Some(msg) => msg.id.to_owned(),
            None => String::default().as_str().to_string(),
        };

        let thread_id = match reply_to {
            Some(msg) => msg.thread_id,
            None => String::default().as_str().to_string(),
        };

        Message {
            from: from.to_string(),
            to: to.to_string(),
            reply_to: reply_id.to_string(),
            body: body.to_string(),
            id: String::default().as_str().to_string(),
            thread_id: thread_id.to_string(),
            timestamp: 0,
            attachments: attachments.to_owned(),
            subject: match reply_subject.is_empty() {
                true => subject.to_string(),
                _ => reply_subject,
            },
        }
    }

    pub fn reply(&self, body: &str, attachments: &Vec<String>) -> Envelope {
        Envelope::make(
            &self.from,
            &self.subject,
            body,
            attachments,
            Some(self.clone().to_owned()),
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Envelope {
    pub to: String,
    pub subject: String,
    pub body: String,
    pub reply_to: String,
    pub thread_id: String,
    pub attachments: Vec<String>,
}

impl Envelope {
    pub fn make(
        to: &str,
        subject: &str,
        body: &str,
        attachments: &Vec<String>,
        reply_to: Option<Message>,
    ) -> Self {
        let reply_subject = match &reply_to {
            Some(msg) => msg.subject.to_owned(),
            None => String::default().as_str().to_string(),
        };

        let reply_id = match &reply_to {
            Some(msg) => msg.id.to_owned(),
            None => String::default().as_str().to_string(),
        };

        let thread_id = match reply_to {
            Some(msg) => msg.thread_id,
            None => String::default().as_str().to_string(),
        };

        Envelope {
            to: to.to_string(),
            reply_to: reply_id.to_string(),
            body: body.to_string(),
            thread_id: thread_id.to_string(),
            attachments: attachments.to_owned(),
            subject: match reply_subject.is_empty() {
                true => subject.to_string(),
                _ => reply_subject,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SubjBody {
    pub subject: String,
    pub body: String,
}

impl SubjBody {
    pub fn to_envelope(&self) -> Envelope {
        Envelope {
            subject: self.subject.to_owned(),
            body: self.body.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Body {
    pub body: String,
}

impl Body {
    pub fn to_envelope(&self) -> Envelope {
        Envelope {
            body: self.body.to_owned(),
            ..Default::default()
        }
    }
}
