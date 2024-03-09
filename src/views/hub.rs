use anyhow::Result;
use poise::{serenity_prelude::CreateEmbed, CreateReply, ReplyHandle};

use crate::types::ApplicationContext;

pub async fn show(ctx: ApplicationContext<'_>, message: &ReplyHandle) -> Result<()> {
    let embed = CreateEmbed::new().description("The hub");
    let m = CreateReply::default().embed(embed);

    message.edit(ctx, m).await?;

    Ok(())
}
