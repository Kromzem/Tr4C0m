use std::time::Duration;

use crate::{
    api::agent,
    types::{ApplicationContext, Data},
};
use anyhow::{Error, Result};
use poise::{execute_modal, serenity_prelude::ComponentInteractionCollector, Modal};
use serenity::builder::CreateCommand;

#[poise::command(slash_command, ephemeral)]
pub async fn login(ctx: ApplicationContext<'_>) -> Result<()> {
    let token = get_user_token(ctx).await?;
    let agent = agent::get_agent(&token).await?.data;

    ctx.data.save_user_token(ctx.author().id.0, &token);

    ctx.send(|m| {
        m.ephemeral(true).embed(|e| {
            e.description(format!(
                "Login successful!\nWelcome back '{}' :)",
                &agent.symbol
            ))
        })
    })
    .await?;

    Ok(())
}

async fn get_user_token(ctx: ApplicationContext<'_>) -> Result<String> {
    let modal = execute_modal::<Data, Error, TokenModal>(ctx, None, Some(Duration::from_secs(360)))
        .await?
        .ok_or(Error::msg("No token received!".to_string()))?;

    Ok(modal.token)
}

#[derive(Modal)]
#[name = "Enter your agent token"]
struct TokenModal {
    #[placeholder = "Your token"]
    #[name = "Token"]
    token: String,
}

pub fn register() -> CreateCommand {
    CreateCommand::new("login").description("Perform login")
}
