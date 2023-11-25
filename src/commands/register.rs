use std::time::Duration;

use poise::execute_modal_on_component_interaction;
use poise::futures_util::stream::Collect;
use poise::serenity_prelude::model::application::component::ActionRowComponent;
use poise::serenity_prelude::{CollectComponentInteraction, CreateSelectMenuOption};
use poise::{serenity_prelude::ButtonStyle, ReplyHandle};
use space_traders::apis::default_api;
use space_traders::models::{Faction, FactionSymbols, Register201Response};
use space_traders::{
    apis::{
        factions_api::{self, GetFactionsSuccess},
        Configuration,
    },
    models::GetFactions200Response,
};

use crate::api::factions::get_factions;
use crate::types::{Context, Error};

#[poise::command(slash_command, ephemeral)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;

    let reply = ctx
        .send(|m| {
            m.ephemeral(true)
                .embed(|e| e.description("Welcome :)\nPlease wait ..."))
        })
        .await?;

    let api_config = Configuration::new();
    let factions = get_factions(&api_config).await?;
    let faction_index = select_faction(ctx, &reply, &factions).await?;
    let login_creds = select_callsign(ctx, &reply).await?;
    let token =
        perform_registration(&factions[faction_index], login_creds.0, login_creds.1).await?;

    show_registration_result(ctx, &reply, token).await?;

    Ok(())
}

async fn select_faction(
    ctx: Context<'_>,
    reply: &ReplyHandle<'_>,
    factions: &Vec<Faction>,
) -> Result<usize, Error> {
    reply
        .edit(ctx, |m| {
            m.embed(|e| e.description("Welcome :)\nChoose a faction:"))
                .components(|c| {
                    c.create_action_row(|r| {
                        r.create_select_menu(|menu| {
                            menu.placeholder("Select faction")
                                .options(|opts| {
                                    opts.set_options(
                                        factions
                                            .iter()
                                            .enumerate()
                                            .map(|(i, f)| {
                                                CreateSelectMenuOption::new(&f.name, i.to_string())
                                            })
                                            .collect(),
                                    )
                                })
                                .custom_id("faction")
                        })
                    })
                })
        })
        .await?;

    let interaction = CollectComponentInteraction::new(ctx.serenity_context())
        .timeout(Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "faction")
        .await
        .ok_or(Error::from("Timeout".to_string()))?;
    interaction.defer(ctx).await?;

    Ok(usize::from_str_radix(&interaction.data.values[0], 10)?)
}

async fn get_faction_options() -> Result<Vec<CreateSelectMenuOption>, Error> {
    let config = Configuration::new();
    let result = factions_api::get_factions(&config, Some(1), Some(20)).await?;

    let GetFactionsSuccess::Status200(GetFactions200Response { data, meta: _ }) = result.content;

    Ok(data
        .iter()
        .map(|f| CreateSelectMenuOption::new(&f.name, (f.symbol as u8).to_string()))
        .collect())
}

async fn select_callsign(
    ctx: Context<'_>,
    reply: &ReplyHandle<'_>,
) -> Result<(String, Option<String>), Error> {
    reply
        .edit(ctx, |m| {
            m.embed(|e| e.description("Enter your player name:"))
                .components(|c| {
                    c.create_action_row(|r| {
                        r.create_button(|b| {
                            b.style(ButtonStyle::Primary)
                                .label("Enter name")
                                .custom_id("name")
                        })
                    })
                })
        })
        .await?;

    let interaction = CollectComponentInteraction::new(ctx.serenity_context())
        .timeout(Duration::from_secs(120))
        .filter(move |mci| mci.data.custom_id == "name")
        .await
        .ok_or(Error::from("Timeout".to_string()))?;

    let modal = execute_modal_on_component_interaction::<NameModal>(
        ctx,
        interaction,
        None,
        Some(Duration::from_secs(360)),
    )
    .await?
    .ok_or(Error::from("No name received!".to_string()))?;

    Ok((modal.name, modal.email))
}

async fn perform_registration(
    faction: &Faction,
    name: String,
    email: Option<String>,
) -> Result<String, Error> {
    let config = Configuration::new();

    let register_result = default_api::register(
        &config,
        Some(space_traders::models::RegisterRequest {
            faction: faction.symbol,
            symbol: name,
            email,
        }),
    )
    .await?;

    let default_api::RegisterSuccess::Status201(Register201Response { data }) =
        register_result.content;

    Ok(data.token)
}

async fn show_registration_result(
    ctx: Context<'_>,
    reply: &ReplyHandle<'_>,
    token: String,
) -> Result<(), Error> {
    reply
        .edit(ctx, |m| m
            .embed(|e| e
                .description("Registered successfully!\nUse the token below to log into your agent to start playing ;)")
                .field("Token", &token, true)
            )
        )
        .await?;

    Ok(())
}

#[derive(poise::Modal)]
#[name = "Enter agent name"]
struct NameModal {
    #[placeholder = "SP4C3_TR4D3R"]
    #[name = "Agent name"]
    #[min_length = 3]
    #[max_length = 14]
    name: String,

    #[placeholder = "space@trader.com"]
    #[name = "E-Mail"]
    email: Option<String>,
}
