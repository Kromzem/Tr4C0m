use crate::{
    state::{get_view_token, save_view_token},
    types::DiscordContext,
    util::util::show_message,
    views::{authenticate::AuthenticationViewController, view::View},
};
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

pub async fn handle(ctx: &DiscordContext, user_id: u64, interaction_token: &str) -> Result<()> {
    // let embed = CreateEmbed::new().description("This will be the view");
    // let view = EditInteractionResponse::new().embed(embed);

    // show_message(ctx, &view, interaction_token).await?;

    // let embed_2 = CreateEmbed::new().description("This will be the view - edited");
    // let view_2 = EditInteractionResponse::new().embed(embed_2);
    // show_message(ctx, &view_2, interaction_token).await?;

    // bail!("Test error")

    if let Some(token) = get_view_token(user_id) {
        let message = EditInteractionResponse::new()
            .embed(CreateEmbed::new().description("Another view was opened!"))
            .components(Vec::with_capacity(0));

        //this is not necessary to be completed :)
        let _ = show_message(ctx, &message, &token).await;
    }

    save_view_token(user_id, interaction_token);

    let view = View::Authentication(AuthenticationViewController::new(user_id));
    view.show(ctx).await?;

    Ok(())
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
