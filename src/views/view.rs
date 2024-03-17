use anyhow::{anyhow, Result};
use serenity::all::ComponentInteraction;

use crate::types::DiscordContext;

use super::authenticate::AuthenticationViewController;

// pub trait View {
//     async fn handle_button(&self, fctx: DiscordContext, btn_id: &str) -> Result<()>;
//     async fn handle_modal(&self, ctx: DiscordContext) -> Result<()>;
//     async fn handle_select_menu(&self, ctx: DiscordContext) -> Result<()>;
// }

pub enum View {
    Authentication(AuthenticationViewController),
}

impl View {
    pub async fn handle_button(&self, ctx: &DiscordContext, button_id: &str) -> Result<()> {
        match self {
            View::Authentication(controller) => controller.handle_button(ctx, button_id).await,
            _ => Err(anyhow!("Unable to handle button!")),
        }
    }

    pub async fn show(&self, ctx: &DiscordContext) -> Result<()> {
        match self {
            View::Authentication(controller) => controller.show(ctx).await,
            _ => Err(anyhow!("View can't be shown!")),
        }
    }
}
