use anyhow::{anyhow, Result};
use serenity::builder::{
    CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, EditInteractionResponse,
};

use crate::{
    types::DiscordContext,
    util::util::{show_message, show_message_for_user},
};

pub async fn authenticate(ctx: DiscordContext, user_id: u64) -> Result<()> {
    let message = EditInteractionResponse::new().embed(CreateEmbed::new().description("Auth"));

    show_message_for_user(&ctx, &message, user_id).await
}

const login_button_id: &'static str = "button_authentication_login";
const registration_button_id: &'static str = "bitton_authentication_registration";

pub struct AuthenticationViewController {
    user_id: u64,
}

impl AuthenticationViewController {
    pub fn new(user_id: u64) -> AuthenticationViewController {
        AuthenticationViewController { user_id }
    }

    pub async fn show(&self, ctx: &DiscordContext) -> Result<()> {
        let message = EditInteractionResponse::new()
            .embed(self.get_embed())
            .components(self.get_components());

        show_message_for_user(ctx, &message, self.user_id).await
    }

    fn get_embed(&self) -> CreateEmbed {
        CreateEmbed::new().description("Authentication view")
    }

    fn get_components(&self) -> Vec<CreateActionRow> {
        let mut rows = Vec::new();

        let mut buttons = Vec::new();
        buttons.push(
            CreateButton::new(login_button_id)
                .style(serenity::all::ButtonStyle::Primary)
                .label("Login"),
        );
        buttons.push(
            CreateButton::new(registration_button_id)
                .style(serenity::all::ButtonStyle::Secondary)
                .label("Register"),
        );

        rows.push(CreateActionRow::Buttons(buttons));

        rows
    }

    pub async fn handle_button(&self, ctx: &DiscordContext, button_id: &str) -> Result<()> {
        match button_id {
            login_button_id => self.login().await,
            registration_button_id => self.register().await,
            _ => Err(anyhow!("Invalid button id")),
        }
    }

    async fn login(&self) -> Result<()> {
        todo!()
    }

    async fn register(&self) -> Result<()> {
        todo!()
    }
}
