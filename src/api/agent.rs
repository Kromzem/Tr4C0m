use reqwest::Method;

use super::{
    models::agent::Agent,
    space_traders::{perform_api_request, ApiData, ApiRequestData},
};
use anyhow::Result;

pub async fn get_agent(token: &str) -> Result<ApiData<Agent>> {
    perform_api_request(ApiRequestData::new(Method::GET, "my/agent").authorization(token)).await
}
