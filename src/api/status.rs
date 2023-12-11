use reqwest::Method;
use serde::Deserialize;

use crate::types::Error;

use super::space_traders::{perform_api_request, ApiRequestData};

pub async fn get_status() -> Result<Status, Error> {
    perform_api_request(ApiRequestData::new(Method::GET, "")).await
}

#[derive(Deserialize)]
pub struct Status {
    pub status: String,
    pub version: String,
    pub description: String,
    pub stats: Stats,
    pub leaderboards: Leaderboards,
    #[serde(rename = "serverResets")]
    pub server_reset: ServerReset,
    pub announcements: Vec<Announcement>,
}

#[derive(Deserialize)]
pub struct Stats {
    pub agents: u32,
    pub ships: u32,
    pub systems: u32,
    pub waypoints: u32,
}

#[derive(Deserialize)]
pub struct Leaderboards {
    #[serde(rename = "mostCredits")]
    pub most_credits: Vec<MostCreditsLeaderboardEntry>,
}

#[derive(Deserialize)]
pub struct MostCreditsLeaderboardEntry {
    #[serde(rename = "agentSymbol")]
    pub agent_symbol: String,
    pub credits: i64,
}

#[derive(Deserialize)]
pub struct ServerReset {
    pub next: String,
    pub frequency: String,
}

#[derive(Deserialize)]
pub struct Announcement {
    pub title: String,
    pub body: String,
}
