use crate::types::Context;
use anyhow::Result;

#[poise::command(slash_command, ephemeral)]
pub async fn logout(ctx: Context<'_>) -> Result<()> {
    ctx.data().delete_user_token(ctx.author().id.0);

    ctx.send(|m| {
        m.ephemeral(true)
            .embed(|e| e.description("Logout successful!\nBye :)"))
    })
    .await?;

    Ok(())
}
