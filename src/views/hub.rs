use anyhow::{anyhow, Result};
use serenity::builder::{CreateEmbed, EditInteractionResponse};

use crate::{api::agent::get_agent, state::get_space_traders_token};

// pub async fn show(ctx: ApplicationContext<'_>, message: &ReplyHandle) -> Result<()> {
//     let embed = CreateEmbed::new().description("The hub");
//     let m = CreateReply::default().embed(embed);

//     message.edit(ctx, m).await?;

//     Ok(())
// }

pub struct HubViewController {}

impl HubViewController {
    pub fn new() -> HubViewController {
        HubViewController {}
    }

    pub async fn get_view(&self, user_id: u64) -> Result<EditInteractionResponse> {
        let token = get_space_traders_token(user_id)?;
        let agent = get_agent(&token).await?;

        Ok(
            EditInteractionResponse::new().embed(CreateEmbed::new().description(format!(
                "Hello '{}' :)\n\nThis is the main hub!",
                agent.data.account_id
            ))),
        )
    }
}
