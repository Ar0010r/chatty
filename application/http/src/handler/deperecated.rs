
use crate::system::email::{gmail::service::Service, shared::models::Message};
use axum::{extract::Path, response::IntoResponse, Json};


pub async fn send(message: Json<Message>)-> impl IntoResponse {
    let service = Service::build(message.from.clone()).await;
    let result = service.send(&message.to, &message.subject, &message.body, &message.attachments, Some(&message.reply_to)).await;

    let response= match result {
         Ok((_, msg)) => serde_json::to_string(&msg).unwrap(),
         Err(e) => format!("{:?}", e)
    };

    Json(response)
}

pub async fn list()-> impl IntoResponse {
    let service = Service::build("support@zhsrecruitment.com".to_string()).await;
    let result = service.unread_list().await;

    Json(result)
}

pub async fn sent_list()-> impl IntoResponse {
    let service = Service::build("support@zhsrecruitment.com".to_string()).await;
    let result = service.sent_list().await;

    Json(result)
}

pub async fn show(Path(id): Path<String>)-> impl IntoResponse {
    let service = Service::build("support@zhsrecruitment.com".to_string()).await;
    let result = service.show(&id).await;

    let response= match result {
        Some(msg) => serde_json::to_string(&msg).unwrap(),
        None => format!("No message found with id: {}", id)
    };

    Json(response)
}

pub async fn conversation(Path(email): Path<String>)-> impl IntoResponse {
    let service = Service::build("support@zhsrecruitment.com".to_string()).await;
    let result = service.get_conversation(&email).await;

    Json(result)
}

