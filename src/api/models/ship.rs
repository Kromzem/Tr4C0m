use serde::Deserialize;

#[derive(Deserialize)]
pub struct Ship {
    pub symbol: String,
    pub registration: Registration,
    pub nav: Nav,
    // pub crew: Crew,
    // pub frame: Frame,
    // pub reactor: Reactor,
    // pub engine: Engine,
    // pub cooldown: Cooldown,
    // pub modules: Vec<Module>,
    // pub mounts: Vec<Mount>,
    // pub cargo: CargoDetails,
    // pub fuel: Fuel,
}

#[derive(Deserialize)]
pub struct Registration {
    pub name: String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct Nav {
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    #[serde(rename = "waypointSymbol")]
    pub waypoint_symbol: String,
    pub route: Route,
    pub status: String,
    #[serde(rename = "flightMode")]
    pub flight_mode: String,
}

#[derive(Deserialize)]
pub struct Route {
    pub departure: Waypoint,
    pub origin: Waypoint,
    #[serde(rename = "departureTime")]
    pub departure_time: String,
    pub arrival: String,
}

#[derive(Deserialize)]
pub struct Waypoint {
    pub symbol: String,
    #[serde(rename = "type")]
    pub waypoint_type: String,
    #[serde(rename = "systemSymbol")]
    pub system_symbol: String,
    pub x: i64,
    pub y: i64,
}
