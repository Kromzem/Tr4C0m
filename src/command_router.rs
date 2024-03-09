use std::env;

use anyhow::{bail, Result};
use serenity::{
    all::{CommandInteraction, GuildId},
    builder::EditInteractionResponse,
};

use crate::{
    commands::{ping, play},
    types::DiscordContext,
    util::util::show_message,
};

pub async fn register_commands(ctx: DiscordContext) -> Result<()> {
    read_dev_guild_id()
        .set_commands(ctx.http, vec![play::register(), ping::register()])
        .await?;

    Ok(())
}

pub async fn handle_command(ctx: DiscordContext, command: CommandInteraction) -> Result<()> {
    command.defer_ephemeral(&ctx.http).await?;

    if let Err(error) = perform_command(&ctx, &command).await {
        let message = EditInteractionResponse::new().content(format!(
            "```diff\n- {}\n```",
            error.to_string().replace("\n", "- \n")
        ));

        show_message(&ctx, &message, &command.token).await?;
    }

    Ok(())
}

async fn perform_command(ctx: &DiscordContext, command: &CommandInteraction) -> Result<()> {
    match command.data.name.as_str() {
        ping::IDENTIFIER => ping::handle(&ctx, &command.token).await,
        play::IDENTIFIER => play::handle(&ctx, &command.token).await,
        _ => bail!("Unknown command '{}'!", &command.data.name),
    }
}

fn read_dev_guild_id() -> GuildId {
    GuildId::new(
        u64::from_str_radix(&env::var("dev_guild_id").expect("No dev guild id set"), 10)
            .expect("invalid dev guild id"),
    )
}
