use serde::Deserialize;

#[derive(Deserialize)]
pub struct Waypoint {
    pub symbol: String,
    #[serde(rename = "type")]
    pub waypoint_type: String,
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    pub x: i64,
    pub y: i64,
    pub orbitals: Vec<WaypointSymbol>,
    pub orbits: Option<String>,
}

#[derive(Deserialize)]
pub struct WaypointSymbol {
    pub symbol: String,
}
