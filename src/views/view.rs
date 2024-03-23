use std::collections::HashMap;

use anyhow::{anyhow, bail, Error, Result};
use serenity::{
    all::{
        ButtonKind, Command, ComponentInteraction, ComponentInteractionDataKind, Interaction,
        InteractionCreateEvent, InteractionType, ModalInteraction,
    },
    builder::{CreateEmbed, EditInteractionResponse},
    futures::TryFutureExt,
    model::Color,
};

use crate::{
    data::modal::ModalData,
    state::{get_current_view, get_view_token},
    types::DiscordContext,
    util::util::{show_interaction_error, show_message_for_user},
};

use super::{authenticate::AuthenticationViewController, hub::HubViewController};

// pub trait View {
//     async fn handle_button(&self, fctx: DiscordContext, btn_id: &str) -> Result<()>;
//     async fn handle_modal(&self, ctx: DiscordContext) -> Result<()>;
//     async fn handle_select_menu(&self, ctx: DiscordContext) -> Result<()>;
// }

pub enum View {
    Authentication(AuthenticationViewController),
    Hub(HubViewController),
}

impl View {
    pub async fn handle_button(
        &self,
        ctx: &DiscordContext,
        interaction: ComponentInteraction,
    ) -> Result<()> {
        match self {
            View::Authentication(controller) => controller.handle_button(ctx, interaction).await,
            _ => Err(anyhow!("Unable to handle button!")),
        }
    }

    pub async fn handle_select_menu(
        &self,
        ctx: &DiscordContext,
        menu_id: &str,
        selection: Vec<String>,
    ) -> Result<()> {
        match self {
            _ => Err(anyhow!("Unable to handle select menu!")),
        }
    }

    pub async fn handle_modal(&self, ctx: &DiscordContext, modal_data: ModalData) -> Result<()> {
        match self {
            View::Authentication(controller) => controller.handle_modal(ctx, modal_data).await,
            _ => Err(anyhow!("Unable to handle modal!")),
        }
    }

    pub async fn show(&self, ctx: &DiscordContext, user_id: u64) -> Result<()> {
        let view = self.get_view(user_id).await?;

        show_message_for_user(ctx, &view, user_id).await
    }

    pub async fn show_with_error(
        &self,
        ctx: &DiscordContext,
        user_id: u64,
        error: &Error,
    ) -> Result<()> {
        let view = self.get_view(user_id).await?.add_embed(
            CreateEmbed::new()
                .description(error.to_string())
                .color(Color::from_rgb(0xff, 0x00, 0x00)),
        );

        show_message_for_user(ctx, &view, user_id).await
    }

    async fn get_view(&self, user_id: u64) -> Result<EditInteractionResponse> {
        let view = match self {
            View::Authentication(controller) => controller.get_view(),
            View::Hub(controller) => controller.get_view(user_id).await?,
            _ => bail!("View can't be shown!"),
        };

        Ok(view)
    }
}

pub async fn handle_view_interaction(ctx: &DiscordContext, interaction: Interaction) -> Result<()> {
    let (user_id, result): (u64, Result<()>) = match interaction {
        Interaction::Component(component) => (
            component.user.id.get(),
            handle_view_component_interaction(ctx, component).await,
        ),
        Interaction::Modal(modal) => (
            modal.user.id.get(),
            handle_view_modal_interaction(ctx, modal).await,
        ),
        _ => bail!("Invalid interaction"),
    };

    if let Err(error) = result {
        let view = get_current_view(user_id);
        if let Ok(view) = view {
            let _ = view
                .read()
                .await
                .show_with_error(ctx, user_id, &error)
                .await;
            // let _ = show_interaction_error(ctx, &token, &error).await;        }
        }

        return Err(error);
    }

    Ok(())
}

pub async fn handle_view_component_interaction(
    ctx: &DiscordContext,
    component: ComponentInteraction,
) -> Result<()> {
    // component.defer_ephemeral(ctx).await?;

    let view = get_current_view(component.user.id.get())?;

    if let ComponentInteractionDataKind::Button = component.data.kind {
        return view.read().await.handle_button(ctx, component).await;
    }

    if let ComponentInteractionDataKind::StringSelect { values } = component.data.kind {
        return view
            .read()
            .await
            .handle_select_menu(ctx, &component.data.custom_id, values)
            .await;
    }

    bail!("Invalid component type")
}

pub async fn handle_view_modal_interaction(
    ctx: &DiscordContext,
    modal: ModalInteraction,
) -> Result<()> {
    modal.defer(ctx).await?;

    let view = get_current_view(modal.user.id.get())?;

    return view
        .read()
        .await
        .handle_modal(ctx, ModalData::parse(modal.data))
        .await;
}
