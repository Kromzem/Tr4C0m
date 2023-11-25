use space_traders::{
    apis::{
        factions_api::{self, GetFactionsSuccess},
        Configuration,
    },
    models::{Faction, GetFactions200Response},
};

use crate::types::Error;

pub async fn get_factions(configuration: &Configuration) -> Result<Vec<Faction>, Error> {
    let answer = factions_api::get_factions(configuration, Some(1), Some(20)).await?;

    let GetFactionsSuccess::Status200(GetFactions200Response { data, meta: _ }) = answer.content;

    Ok(data)
}
