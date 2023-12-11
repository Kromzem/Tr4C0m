use serde::Deserialize;

#[derive(Deserialize)]
pub struct Contract {
    pub id: String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub terms: Terms,
    pub accepted: bool,
    pub fulfilled: bool,
    #[serde(rename = "deadlineToAccept")]
    pub deadline_to_accept: String,
}

#[derive(Deserialize)]
pub struct Terms {
    pub deadline: String,
    pub payment: Payment,
    pub deliver: Vec<Cargo>,
}

#[derive(Deserialize)]
pub struct Payment {
    #[serde(rename = "onAccepted")]
    pub on_accepted: i64,
    #[serde(rename = "onFulfilled")]
    pub on_fulfilled: i64,
}

#[derive(Deserialize)]
pub struct Cargo {
    #[serde(rename = "tradeSymbol")]
    pub trade_symbol: String,
    #[serde(rename = "destinationSymbol")]
    pub destination_symbol: String,
    #[serde(rename = "unitsRequired")]
    pub units_required: i32,
    #[serde(rename = "unitsFulfilled")]
    pub units_fulfilled: i32,
}
