use serde::Deserialize;

use super::{faction::FactionSymbol, waypoint::Waypoint};

#[derive(Deserialize)]
pub struct System {
    pub symbol: String,
    #[serde(rename = "sectorSymbol")]
    pub sector_symbol: String,
    #[serde(rename = "type")]
    pub system_type: String,
    pub x: i64,
    pub y: i64,
    pub waypoints: Vec<Waypoint>,
    pub factions: Vec<FactionSymbol>,
}
