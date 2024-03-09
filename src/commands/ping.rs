use anyhow::Result;
use std::time::Instant;

use serenity::{
    all::{CommandInteraction, Message, MessageAction},
    async_trait,
    builder::{CreateCommand, EditInteractionResponse, EditMessage},
    prelude::Context,
};

use crate::{types::DiscordContext, util::util::show_message};

use super::handler::CommandHandler;

pub const IDENTIFIER: &'static str = "ping";

pub fn register() -> CreateCommand {
    CreateCommand::new(IDENTIFIER).description("Tests bot response time")
}

pub async fn handle(ctx: &DiscordContext, interaction_token: &str) -> Result<()> {
    let mut content = EditInteractionResponse::new().content("Pinging ...");

    let watch = Instant::now();
    show_message(ctx, &content, interaction_token).await?;
    // message.edit(&ctx.http, content).await?;

    let duration = watch.elapsed().as_millis();

    content = EditInteractionResponse::new().content(format!("Ping took {duration}ms"));

    show_message(ctx, &content, interaction_token).await?;
    // message.edit(&ctx.http, content).await?;

    Ok(())
}

// pub struct PingCommandHandler;

// #[async_trait]
// impl CommandHandler for PingCommandHandler {
//     fn get_identifier(&self) -> &str {
//         IDENTIFIER
//     }

//     fn get_command_builder(&self) -> CreateCommand {
//         CreateCommand::new(IDENTIFIER).description("Tests bot response time")
//     }

//     async fn perform(&self, ctx: &DiscordContext, interaction_token: &str) -> Result<()> {
//         let mut content = EditInteractionResponse::new().content("Pinging ...");

//         let watch = Instant::now();
//         show_message(ctx, &content, interaction_token).await?;
//         // message.edit(&ctx.http, content).await?;

//         let duration = watch.elapsed().as_millis();

//         content = EditInteractionResponse::new().content(format!("Ping took {duration}ms"));

//         show_message(ctx, &content, interaction_token).await?;
//         // message.edit(&ctx.http, content).await?;

//         Ok(())
//     }
// }
