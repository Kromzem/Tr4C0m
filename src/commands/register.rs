use std::time::Duration;

use crate::api::factions::list_factions;
use crate::api::models::faction::Faction;
use crate::api::register;
use crate::types::{ApplicationContext, Context};
use crate::views::view::{UuidButtonCreate, View, ViewBuilder};

use anyhow::{Error, Result};
use serenity::builder::CreateCommand;

#[poise::command(slash_command, ephemeral)]
pub async fn perform(ctx: ApplicationContext<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let mut view = View::new(ctx);

    view.display(welcome_view()).await?;

    let reply = ctx.send(welcome_view()).await?;

    let faction_symbol = select_faction(ctx, &reply).await?;
    let login_creds = select_callsign(ctx, &reply).await?;
    let token =
        perform_registration(&faction_symbol, &login_creds.0, login_creds.1.as_deref()).await?;

    show_registration_result(ctx, &reply, token).await?;

    Ok(())
}

async fn choose_name_view(view: &mut View) {
    let embed = CreateEmbed::new()
        .description("Welcome to SpaceTraders :)\nHow dou you want to be called?");

    let mut buttons = Vec::with_capacity(1);
    buttons.push((CreateButton::with_uuid(), test));

    let mut builder = ViewBuilder::new(embed).add_buttons_row(buttons);
}

fn test() {}

async fn select_faction(ctx: Context<'_>, reply: &ReplyHandle<'_>) -> Result<String> {
    let factions = list_factions(20, 1).await?.data;

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
                                            .map(|f| {
                                                CreateSelectMenuOption::new(&f.name, &f.symbol)
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
        .ok_or(Error::msg("Timeout".to_string()))?;
    interaction.defer(ctx).await?;

    Ok(interaction.data.values[0].to_string())
}

async fn select_callsign(
    ctx: Context<'_>,
    reply: &ReplyHandle<'_>,
) -> Result<(String, Option<String>)> {
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
        .ok_or(Error::msg("Timeout".to_string()))?;

    let modal = execute_modal_on_component_interaction::<NameModal>(
        ctx,
        interaction,
        None,
        Some(Duration::from_secs(360)),
    )
    .await?
    .ok_or(Error::msg("No name received!".to_string()))?;

    Ok((modal.name, modal.email))
}

async fn perform_registration(
    faction_symbol: &str,
    name: &str,
    email: Option<&str>,
) -> Result<String> {
    Ok(register::register(name, faction_symbol, email).await?.token)
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

pub fn register() -> CreateCommand {
    CreateCommand::new("register")
}
