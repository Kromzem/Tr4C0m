use anyhow::{bail, Result};
use serenity::{all::ButtonStyle, builder::CreateButton};
use uuid::Uuid;

use crate::{types::DiscordContext, util::util::{show_message, show_message_for_user}};

use super::view::View;

pub struct HomeView {
    login_btn_id: String,
    register_btn_id: String,
}

impl View for HomeView {
    async fn handle_button(
        &self,
        ctx: crate::types::DiscordContext,
        btn_id: &str,
    ) -> anyhow::Result<()> {
        if btn_id == self.login_btn_id {
            return self.login(ctx);
        }

        if btn_id == self.register_btn_id {
            return self.register(ctx);
        }

        bail!("Unhandled button!")
    }

    async fn handle_modal(&self, ctx: crate::types::DiscordContext) -> anyhow::Result<()> {
        bail!("Unhandled modal")
    }

    async fn handle_select_menu(&self, ctx: crate::types::DiscordContext) -> anyhow::Result<()> {
        bail!("Unhandled select menu")
    }
}

impl HomeView {
    pub fn new() -> HomeView {
        HomeView {
            register_btn_id: Uuid::new_v4().to_string(),
            login_btn_id: Uuid::new_v4().to_string(),
        }
    }

    fn login_button(&self) -> CreateButton {
        CreateButton::new(self.login_btn_id)
            .label("Login")
            .style(ButtonStyle::Primary)
    }

    fn register_button(&self) -> CreateButton {
        CreateButton::new(self.register_btn_id)
            .label("Register")
            .style(ButtonStyle::Secondary)
    }

    fn login(&self, ctx: DiscordContext) -> Result<()> {
        show_message_for_user(, , )
        
        Ok(())
    }

    fn register(&self, ctx: DiscordContext) -> Result<()> {
        Ok(())
    }
}
