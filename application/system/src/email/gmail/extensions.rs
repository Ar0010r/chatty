use google_gmail1::api::{
    Message as GmailMessage, MessagePart, MessagePartBody, MessagePartHeader,
};
use lettre::{
    message::{header, Body, MultiPart, SinglePart},
    Message as LettreMessage,
};
use std::fs;

use crate::email::{
    retrieve_from_str,
    shared::{
        models::{Envelope, Message},
        traits::{MessageConvertible, MessageParts},
    },
};

impl MessageConvertible<GmailMessage> for GmailMessage {
    fn to_message(&self) -> Message {
        Message {
            from: self.get_from(),
            to: self.get_to(),
            subject: self.get_subject(),
            body: self.get_body(),
            reply_to: self.get_reply_to(),
            id: self.get_id(),
            thread_id: self.get_thread_id(),
            attachments: Vec::new(),
            timestamp: self.get_timestamp(),
        }
    }

    fn from_message(mesage: Message) -> GmailMessage {
        GmailMessage {
            id: Some(mesage.id),
            payload: Some(Default::default()),
            ..Default::default()
        }
    }
}

impl MessageParts for GmailMessage {
    fn get_id(&self) -> String {
        self.id.as_ref().unwrap_or(&String::default()).to_string()
    }

    fn get_from(&self) -> String {
        let email = get_header_value(self.clone(), "From");
        retrieve_from_str(&email).unwrap_or_default()
    }

    fn get_to(&self) -> String {
        let email = get_header_value(self.clone(), "To");
        retrieve_from_str(&email).unwrap_or_default()
    }

    fn get_subject(&self) -> String {
        get_header_value(self.clone(), "Subject")
    }

    fn get_reply_to(&self) -> String {
        get_header_value(self.clone(), "In-Reply-To")
    }

    fn get_thread_id(&self) -> String {
        self.thread_id
            .as_ref()
            .unwrap_or(&String::default())
            .to_string()
    }

    fn get_body(&self) -> String {
        let result = get_body_from_parts(self);
        if result == String::default() {
            return get_body_from_root(self);
        }

        result
    }

    fn get_timestamp(&self) -> i64 {
        self.internal_date.unwrap_or(0)
    }
}

fn get_header_value(msg: GmailMessage, name: &str) -> String {
    let default = MessagePartHeader::default();
    let headers = msg.payload.as_ref().unwrap().headers.as_ref().unwrap();
    let header = headers
        .iter()
        .find(|h| h.name.as_ref().unwrap() == name)
        .unwrap_or(&default);

    header
        .value
        .as_ref()
        .unwrap_or(&String::default())
        .to_string()
}

fn get_body_from_parts(msg: &GmailMessage) -> String {
    let default = MessagePart::default();
    let default_body = MessagePartBody::default();
    let default_vec = Vec::<MessagePart>::new();
    let default_v = Vec::<u8>::new();

    let payload = msg.payload.as_ref().unwrap_or(&default);
    let parts = payload.parts.as_ref().unwrap_or(&default_vec);
    let body_part = parts
        .iter()
        .find(|p| p.mime_type.as_ref().unwrap() == "text/plain")
        .unwrap_or(&default);
    let body = body_part.body.as_ref().unwrap_or(&default_body);
    let data = body.data.as_ref().unwrap_or(&default_v);

    String::from_utf8(data.to_vec()).unwrap_or_default()
}

fn get_body_from_root(msg: &GmailMessage) -> String {
    let default = MessagePart::default();
    let default_body = MessagePartBody::default();
    let default_v = Vec::<u8>::new();

    let payload = msg.payload.as_ref().unwrap_or(&default);
    let body = payload.body.as_ref().unwrap_or(&default_body);
    let data = body.data.as_ref().unwrap_or(&default_v);

    String::from_utf8(data.to_vec()).unwrap_or_default()
}

impl MessageConvertible<LettreMessage> for LettreMessage {
    fn to_message(&self) -> Message {
        Message {
            from: String::default(),
            to: String::default(),
            subject: String::default(),
            body: String::default(),
            reply_to: String::default(),
            id: String::default(),
            thread_id: String::default(),
            attachments: Vec::new(),
            timestamp: 0,
        }
    }

    fn from_message(mesage: Message) -> LettreMessage {
        let mut builder = LettreMessage::builder();
        let attachments: Vec<Body> = mesage
            .attachments
            .iter()
            .map(|path| Body::new(fs::read(path).unwrap()))
            .collect();

        if mesage.reply_to != String::default() {
            builder = builder
                .in_reply_to(mesage.reply_to.parse().unwrap())
                .references(mesage.reply_to.parse().unwrap());
        }

        let mut body = MultiPart::mixed().singlepart(
            SinglePart::builder()
                .header(header::ContentType::parse("text/plain; charset=utf8").unwrap())
                .body(mesage.body.to_string()),
        );

        for attachment in attachments {
            body = body.singlepart(
                SinglePart::builder()
                    .header(header::ContentType::parse("application/pdf").unwrap())
                    .header(header::ContentDisposition::attachment("agreement.pdf"))
                    .body(attachment),
            );
        }

        builder
            .from(mesage.from.parse().unwrap())
            .to(mesage.to.parse().unwrap())
            .subject(mesage.subject.clone())
            .multipart(body)
            .unwrap()
    }
}

impl MessageConvertible<Envelope> for Envelope {
    fn to_message(&self) -> Message {
        Message {
            to: self.to.clone(),
            subject: self.subject.clone(),
            body: self.body.clone(),
            reply_to: self.reply_to.clone(),
            thread_id: self.thread_id.clone(),
            attachments: self.attachments.clone(),
            ..Default::default()
        }
    }

    fn from_message(mesage: Message) -> Envelope {
        Envelope {
            to: mesage.to,
            subject: mesage.subject,
            body: mesage.body,
            attachments: mesage.attachments,
            reply_to: mesage.reply_to,
            thread_id: mesage.thread_id,
        }
    }
}
