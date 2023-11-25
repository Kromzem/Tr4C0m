use std::time::Duration;

use crate::types::{Context, Error};
use poise::serenity_prelude::interactions::message_component;
use space_traders::{
    apis::{
        factions_api::{get_factions, GetFactionsSuccess},
        Configuration,
    },
    models::GetFactions200Response,
};

#[poise::command(slash_command)]
pub async fn factions(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    let config = Configuration::new();

    let response = get_factions(&config, Some(1), Some(10)).await?;

    let GetFactionsSuccess::Status200(GetFactions200Response { data, meta: _ }) = response.content;

    let message = ctx
        .send(|b| {
            b.content("Select faction: ").components(|c| {
                c.create_action_row(|r| {
                    r.create_select_menu(|m| {
                        m.custom_id("test");
                        m.placeholder("No faction selected");
                        m.options(|opts| {
                            for faction in data.iter() {
                                opts.create_option(|o| {
                                    o.label(&faction.name).value(faction.symbol)
                                });
                            }
                            opts
                        })
                    })
                })
            })
        })
        .await?;

    let interaction = message
        .message()
        .await?
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(60))
        .await;

    let result = match interaction {
        Some(x) => x,
        None => {
            message.message().await?.reply(&ctx, "Timeout").await?;
            return Ok(());
        }
    };

    let selected = &result.data.values[0];

    message
        .edit(ctx, |m| {
            m.content(format!("Selected: {}", &selected))
                .ephemeral(true)
                .components(|c| c)
        })
        .await?;

    Ok(())
}
