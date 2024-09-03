use std::fs;

use google_gmail1::oauth2::ServiceAccountKey;
use std::path::{Path, PathBuf};

const DIR: &str = "/app/src/system/email/gmail/credentials";

pub struct GmailCredentials {
    pub email: String,
    pub key: ServiceAccountKey,
    pub token_location: PathBuf,
}

impl GmailCredentials {
    pub fn new(email: String, key: ServiceAccountKey, token_location: PathBuf) -> Self {
        Self {
            email,
            key,
            token_location,
        }
    }

    pub fn get(email: String) -> Self {
        Self {
            key: get_key(&email),
            token_location: get_token_storage(&email),
            email,
        }
    }
}

pub fn get_key(account: &str) -> ServiceAccountKey {
    let file_name = format!("{}/{}.json", DIR, account);
    print!("file_name {}", file_name);
    let file = std::fs::read_to_string(file_name).unwrap();

    serde_json::from_str::<ServiceAccountKey>(&file).unwrap()
}

pub fn get_token_storage(account: &str) -> std::path::PathBuf {
    let file_name = format!("{}/token-{}.json", DIR, account);

    if !Path::new(&file_name).exists() {
        fs::write(&file_name, "[]").unwrap();
    }

    Path::new(&file_name).to_path_buf()
}
