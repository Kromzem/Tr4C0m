use serde::{de::value::StringDeserializer, Deserialize};

#[derive(Deserialize)]
pub struct Agent {
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub symbol: String,
    pub headquarters: String,
    pub credits: i64,
    #[serde(rename = "startingFaction")]
    pub starting_faction: String,
    #[serde(rename = "shipCount")]
    pub ship_count: usize,
}
