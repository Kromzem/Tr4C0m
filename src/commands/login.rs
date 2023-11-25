use crate::types::{Context, Error};

#[poise::command(slash_command, ephemeral)]
pub async fn login(ctx: Context<'_>, token: String) -> Result<(), Error> {
    Ok(())
}
