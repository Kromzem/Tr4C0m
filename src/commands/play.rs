use crate::{types::DiscordContext, util::util::show_message};
use anyhow::{bail, Result};
use serenity::{
    async_trait,
    builder::{CreateCommand, CreateEmbed, EditInteractionResponse},
};

use super::handler::CommandHandler;

pub const IDENTIFIER: &'static str = "play";

pub fn register() -> CreateCommand {
    CreateCommand::new(IDENTIFIER).description("Let's you play SpaceTraders via Tr4C0m")
}

pub async fn handle(ctx: &DiscordContext, interaction_token: &str) -> Result<()> {
    let embed = CreateEmbed::new().description("This will be the view");
    let view = EditInteractionResponse::new().embed(embed);

    show_message(ctx, &view, interaction_token).await?;

    let embed_2 = CreateEmbed::new().description("This will be the view - edited");
    let view_2 = EditInteractionResponse::new().embed(embed_2);
    show_message(ctx, &view_2, interaction_token).await?;

    bail!("Test error")
}

// pub struct PlayCommandHandler;

//#[async_trait]
// impl PlayCommandHandler {
//     fn get_identifier() -> &str {
//         IDENTIFIER
//     }

//     fn get_command_builder(&self) -> CreateCommand {
//         CreateCommand::new(IDENTIFIER).description("Let's you play SpaceTraders via Tr4C0m")
//     }

//     async fn perform(&self, ctx: &DiscordContext, interaction_token: &str) -> Result<()> {
//         let embed = CreateEmbed::new().description("This will be the view");
//         let view = EditInteractionResponse::new().embed(embed);

//         show_message(ctx, &view, interaction_token).await?;

//         let embed_2 = CreateEmbed::new().description("This will be the view - edited");
//         let view_2 = EditInteractionResponse::new().embed(embed_2);
//         show_message(ctx, &view_2, interaction_token).await?;

//         bail!("Test error")
//     }
// }
