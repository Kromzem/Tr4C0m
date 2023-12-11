use reqwest::Method;

use crate::types::Error;

use super::models::faction::Faction;
use super::space_traders::{perform_api_request, ApiRequestData, PagedData};

pub async fn list_factions(limit: usize, page: usize) -> Result<PagedData<Faction>, Error> {
    perform_api_request(ApiRequestData::new_paged(
        Method::GET,
        "factions",
        limit,
        page,
    ))
    .await
}
