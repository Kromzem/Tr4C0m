use crate::types::{Context, Error};
use poise::command;

#[command(slash_command)]
pub async fn echo(
    ctx: Context<'_>,
    #[description = "Message to be echoed"] message: String,
) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.say(message).await?;

    Ok(())
}
