use std::collections::HashMap;

use axum::extract::Multipart;
use csv::Reader;

use crate::domain::{data::entities::lead::{Model, Status}, service::lead::LeadService};

pub async fn import_csv(mut multipart: Multipart) -> bool {
     let mut leads = HashMap::<String, String>::new();
 
     while let Some(field) = multipart.next_field().await.unwrap() {
         if let Some(name) = field.name() {
             if name == "file" {
                 let data = field.bytes().await.unwrap();
                 let mut rdr = Reader::from_reader(data.as_ref());
                 
                 for result in rdr.deserialize() {
                     let record: String = result.unwrap();
                     let parts = record.split(';').collect::<Vec<&str>>();
                     let email = parts[0];
                     let name = parts[1];
 
                     leads.insert(email.to_string(), name.to_string());
                 }
             }
         }
     }
 
     let mut models = Vec::<Model>::new();
 
     for (email, name) in leads.iter() {
         let lead = Model {
             id: 0,
             hr_company_id: None,
             company_id: None,
             hr_id: None,
             emails: vec![email.to_string()],
             first_name: Some(name.to_string()),
             last_name: Some(String::default()),
             status: Status::None,
             created_at: chrono::Utc::now().naive_utc(),
         };
 
         models.push(lead);
     }
 
     LeadService::insert(models, None).await.is_ok()
 }
 