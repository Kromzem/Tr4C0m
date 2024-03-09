use anyhow::Result;
use poise::serenity_prelude::CreateEmbed;

use crate::{
    api::agent::get_agent,
    commands::checks::get_user_session,
    types::ApplicationContext,
    views::view::{View, ViewBuilder},
};

#[poise::command(slash_command, ephemeral)]
pub async fn me(ctx: ApplicationContext<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let mut view = View::new(ctx);
    let token = get_user_session(ctx).await?;

    let agent = get_agent(&token).await?.data;

    let builder = ViewBuilder::new(
        CreateEmbed::default()
            .field("Agent", agent.symbol, true)
            .field("Account-ID", agent.account_id, true)
            .field("Starting faction", agent.starting_faction, false)
            .field("Headquarters", agent.headquarters, true)
            .field("Credits", agent.credits.to_string(), false)
            .field("Ships", agent.ship_count.to_string(), true),
    );

    view.display(builder).await?;

    Ok(())
}
