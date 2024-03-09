use super::space_traders::{perform_api_request, ApiRequestData};
use anyhow::Result;
use reqwest::Method;
use serde::{Deserialize, Serialize};

pub async fn register(
    name: &str,
    faction_symbol: &str,
    email: Option<&str>,
) -> Result<RegistrationResponseData> {
    perform_api_request::<RegistrationResponse>(
        ApiRequestData::new(Method::POST, "register").content(RegistrationRequest {
            faction: faction_symbol.to_owned(),
            symbol: name.to_owned(),
            email: match email {
                Some(email) => Some(email.to_owned()),
                None => None,
            },
        }),
    )
    .await
    .map(|r| r.data)
}

#[derive(Serialize)]
struct RegistrationRequest {
    faction: String,
    symbol: String,
    email: Option<String>,
}

#[derive(Deserialize)]
struct RegistrationResponse {
    pub data: RegistrationResponseData,
}

#[derive(Deserialize)]
pub struct RegistrationResponseData {
    pub token: String,
}
