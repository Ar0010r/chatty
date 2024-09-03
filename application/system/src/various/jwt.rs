use google_gmail1::hyper::StatusCode;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::{config, exception::model::Exception};

#[derive(Serialize, Deserialize, Debug)]
pub struct Cliams {
    pub login: String,
}

pub fn encode(claims: Cliams) -> Result<String, Exception> {
    let header = Header::default();
    let secret = config::get_jwt().secret.clone();
    let key = EncodingKey::from_secret(secret.as_ref());

    jsonwebtoken::encode(&header, &claims, &key).map_err(|e| {
        Exception::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "encode errors:".to_owned() + e.to_string().as_str(),
            Default::default(),
        )
    })
}

pub fn decode(token: &str) -> Result<TokenData<Cliams>, Exception> {
    let secret = config::get_jwt().secret.clone();
    let key = DecodingKey::from_secret(secret.as_ref());
    let mut rules = Validation::default();
    rules.validate_exp = false;
    rules.required_spec_claims = Default::default();

    jsonwebtoken::decode(token, &key, &rules).map_err(|e| {
        Exception::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "encode errors:".to_owned() + e.to_string().as_str(),
            Default::default(),
        )
    })
}

pub fn decode_or_default(token: &str) -> TokenData<Cliams> {
    if decode(token).is_err() {
        let a = decode(token);
        println!("Error: {:?}", a);
    }

    match decode(token) {
        Ok(token_data) => token_data,
        Err(_) => TokenData {
            header: Header::default(),
            claims: Cliams {
                login: String::default(),
            },
        },
    }
}

pub fn strip_bearer(token: &str) -> &str {
    let prefix = "Bearer ";

    match token.starts_with(prefix) {
        true => token.strip_prefix(prefix).unwrap_or(token),
        false => token,
    }
}
