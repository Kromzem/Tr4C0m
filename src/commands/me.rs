use anyhow::Result;

use crate::{api::agent::get_agent, commands::checks::get_user_session, types::ApplicationContext};

#[poise::command(slash_command, ephemeral)]
pub async fn me(ctx: ApplicationContext<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let token = get_user_session(ctx).await?;

    let agent = get_agent(&token).await?.data;

    ctx.send(|m| {
        m.ephemeral(true).embed(|e| {
            e.field("Agent", agent.symbol, true)
                .field("Account-ID", agent.account_id, true)
                .field("Starting faction", agent.starting_faction, false)
                .field("Headquarters", agent.headquarters, true)
                .field("Credits", agent.credits.to_string(), false)
                .field("Ships", agent.ship_count.to_string(), true)
        })
    })
    .await?;

    Ok(())
}
