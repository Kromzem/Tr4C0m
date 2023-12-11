use serde::Deserialize;

#[derive(Deserialize)]
pub struct Faction {
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub headquarters: String,
    pub traits: Vec<FactionTrait>,
    #[serde(rename = "isRecruiting")]
    pub is_recruiting: bool,
}

#[derive(Deserialize)]
pub struct FactionTrait {
    pub symbol: String,
    pub name: String,
    pub description: String,
}
