use anyhow::Result;
use serenity::builder::{CreateEmbed, EditInteractionResponse};

use crate::{types::DiscordContext, util::util::show_message_for_user};

pub async fn authenticate(ctx: DiscordContext, user_id: u64) -> Result<()> {
    let message = EditInteractionResponse::new().embed(CreateEmbed::new().description("Auth"));

    show_message_for_user(&ctx, &message, user_id).await
}
