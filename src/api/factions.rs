use reqwest::Method;

use super::models::faction::Faction;
use super::space_traders::{perform_api_request, ApiRequestData, PagedApiData};
use anyhow::Result;

pub async fn list_factions(limit: usize, page: usize) -> Result<PagedApiData<Faction>> {
    perform_api_request(ApiRequestData::new_paged(
        Method::GET,
        "factions",
        limit,
        page,
    ))
    .await
}
