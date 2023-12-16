use anyhow::{Error, Result};

use crate::types::{ApplicationContext, Context};

pub async fn get_user_session(ctx: ApplicationContext<'_>) -> Result<String> {
    if let Some(token) = ctx.data().get_user_token(ctx.author().id.0) {
        return Ok(token);
    }

    // ctx.send(|m| m.ephemeral(true).content("Please login first :)"))
    //     .await?;

    return Err(Error::msg("You're not logged in!"));
}
