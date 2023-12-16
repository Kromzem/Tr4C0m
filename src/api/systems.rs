use anyhow::Result;
use reqwest::Method;

use super::{
    models::{system::System, waypoint::Waypoint},
    space_traders::{perform_api_request, ApiData, ApiRequestData, PagedApiData},
};

pub async fn list(limit: usize, page: usize) -> Result<PagedApiData<System>> {
    perform_api_request(ApiRequestData::new_paged(
        Method::GET,
        "systems",
        limit,
        page,
    ))
    .await
}

pub async fn get(symbol: &str) -> Result<ApiData<System>> {
    perform_api_request(ApiRequestData::new(
        Method::GET,
        format!("systems/{}", symbol).as_str(),
    ))
    .await
}

pub async fn list_waypoints(
    system_symbol: &str,
    limit: usize,
    page: usize,
) -> Result<PagedApiData<Waypoint>> {
    perform_api_request(ApiRequestData::new_paged(
        Method::GET,
        &format!("systems/{}/waypoints", system_symbol),
        limit,
        page,
    ))
    .await
}

pub async fn get_waypoint(waypoint_symbol: &str) -> Result<ApiData<Waypoint>> {
    let system_symbol = waypoint_symbol
        .rsplit_once('-')
        .expect("Invalid waypoint symbol format")
        .0;

    perform_api_request(ApiRequestData::new(
        Method::GET,
        &format!("systems/{}/waypoints/{}", system_symbol, waypoint_symbol),
    ))
    .await
}
