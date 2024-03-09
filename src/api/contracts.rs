use reqwest::Method;

use super::{
    models::contract::Contract,
    space_traders::{perform_api_request, ApiRequestData, PagedApiData},
};
use anyhow::Result;

pub async fn list(token: &str, limit: usize, page: usize) -> Result<PagedApiData<Contract>> {
    perform_api_request(
        ApiRequestData::new(Method::GET, "my/contracts")
            .paged(limit, page)
            .authorization(token),
    )
    .await
}
