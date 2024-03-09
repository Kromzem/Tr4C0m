use anyhow::Result;

use crate::{api::contracts, types::ApplicationContext};

use super::checks::get_user_session;

pub async fn show_contracts(ctx: ApplicationContext<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let token = get_user_session(ctx).await?;
    let contracts = contracts::list(&token, 20, 1).await?;

    Ok(())
}
