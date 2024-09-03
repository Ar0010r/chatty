use crate::exception::model::Exception;

use super::jwt::{encode, Cliams};

#[allow(async_fn_in_trait)]
pub trait Authenticable {
    fn get_claims(&self) -> Cliams;
    async fn validate(&self) -> Result<(), Exception>;
}

pub async fn issue_token(data: impl Authenticable) -> Result<String, Exception> {
    data.validate().await?;

    let token = encode(data.get_claims())?;

    Ok(token)
}
