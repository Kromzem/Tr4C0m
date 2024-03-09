use anyhow::Result;

use crate::types::DiscordContext;

pub trait View {
    async fn handle_button(&self, fctx: DiscordContext, btn_id: &str) -> Result<()>;
    async fn handle_modal(&self, ctx: DiscordContext) -> Result<()>;
    async fn handle_select_menu(&self, ctx: DiscordContext) -> Result<()>;
}
