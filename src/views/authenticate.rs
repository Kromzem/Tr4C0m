use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use serenity::{
    all::ComponentInteraction,
    builder::{
        Builder, CreateActionRow, CreateButton, CreateEmbed, CreateInputText,
        CreateInteractionResponse, CreateInteractionResponseMessage, CreateModal,
        EditInteractionResponse,
    },
    http::CacheHttp,
};

use crate::{
    api::agent::get_agent,
    data::modal::ModalData,
    state::{push_view, save_space_traders_token},
    types::DiscordContext,
    util::util::{create_modal, show_message, show_message_for_user},
};

use super::{hub::HubViewController, view::View};

pub async fn authenticate(ctx: DiscordContext, user_id: u64) -> Result<()> {
    let message = EditInteractionResponse::new().embed(CreateEmbed::new().description("Auth"));

    show_message_for_user(&ctx, &message, user_id).await
}

const LOGIN_BUTTON_ID: &'static str = "button_authentication_login";
const REGISTRATION_BUTTON_ID: &'static str = "button_authentication_registration";
const LOGIN_MODAL_ID: &'static str = "modal_authentication_login";
const LOGIN_MODAL_TEXT_TOKEN_ID: &'static str = "modal_text_authentication_login_token";

pub struct AuthenticationViewController {
    user_id: u64,
}

impl AuthenticationViewController {
    pub fn new(user_id: u64) -> AuthenticationViewController {
        AuthenticationViewController { user_id }
    }

    pub fn get_view(&self) -> EditInteractionResponse {
        EditInteractionResponse::new()
            .embed(self.get_embed())
            .components(self.get_components())
    }

    fn get_embed(&self) -> CreateEmbed {
        CreateEmbed::new().description("Authentication view")
    }

    fn get_components(&self) -> Vec<CreateActionRow> {
        let mut rows = Vec::new();

        let mut buttons = Vec::new();
        buttons.push(
            CreateButton::new(LOGIN_BUTTON_ID)
                .style(serenity::all::ButtonStyle::Primary)
                .label("Login"),
        );
        buttons.push(
            CreateButton::new(REGISTRATION_BUTTON_ID)
                .style(serenity::all::ButtonStyle::Secondary)
                .label("Register"),
        );

        rows.push(CreateActionRow::Buttons(buttons));

        rows
    }

    pub async fn handle_button(
        &self,
        ctx: &DiscordContext,
        interaction: ComponentInteraction,
    ) -> Result<()> {
        match interaction.data.custom_id.as_str() {
            LOGIN_BUTTON_ID => self.login(ctx, interaction).await,
            REGISTRATION_BUTTON_ID => self.register().await,
            _ => Err(anyhow!("Invalid button id")),
        }
    }

    pub async fn handle_modal(&self, ctx: &DiscordContext, modal_data: ModalData) -> Result<()> {
        match modal_data.id.as_str() {
            LOGIN_MODAL_ID => self.login_modal(ctx, modal_data.inputs).await,
            _ => Err(anyhow!("Invalid modal id")),
        }
    }

    async fn login(&self, ctx: &DiscordContext, interaction: ComponentInteraction) -> Result<()> {
        let mut components = Vec::new();
        components.push(CreateInputText::new(
            serenity::all::InputTextStyle::Short,
            "Token",
            LOGIN_MODAL_TEXT_TOKEN_ID,
        ));

        let modal = create_modal(LOGIN_MODAL_ID, "Enter your agent token", components);

        interaction
            .create_response(ctx, CreateInteractionResponse::Modal(modal))
            .await?;

        Ok(())
    }

    async fn register(&self) -> Result<()> {
        todo!()
    }

    async fn login_modal(
        &self,
        ctx: &DiscordContext,
        inputs: HashMap<String, String>,
    ) -> Result<()> {
        let token = inputs
            .get(LOGIN_MODAL_TEXT_TOKEN_ID)
            .ok_or(anyhow!("Modal does not contain token data"))?;

        if let Err(error) = get_agent(&token).await {
            return Err(error).context("Login failed with given credentials!");
        }

        save_space_traders_token(self.user_id, &token);

        push_view(self.user_id, View::Hub(HubViewController::new()))
            .read()
            .await
            .show(ctx, self.user_id)
            .await?;

        Ok(())
    }
}
