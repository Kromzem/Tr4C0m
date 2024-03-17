use anyhow::{anyhow, bail, Result};
use serenity::all::{ComponentInteraction, ModalInteraction};

use crate::types::DiscordContext;

pub async fn handle_component(ctx: DiscordContext, component: ComponentInteraction) -> Result<()> {
    let result = match component.data.kind {
        serenity::all::ComponentInteractionDataKind::Button => {
            handle_button(ctx, component.data.custom_id).await
        }
        serenity::all::ComponentInteractionDataKind::StringSelect { values } => {
            handle_select_menu(ctx, component.data.custom_id, values).await
        }
        _ => Err(anyhow!(
            "Unhandled component type: {:?}",
            component.data.kind
        )),
    };

    if let Err(err) = result {
        return Err(err);
    }

    Ok(())
}

async fn handle_button(ctx: DiscordContext, id: String) -> Result<()> {
    bail!("Unhandled button '{}'", id)
}

async fn handle_select_menu(ctx: DiscordContext, id: String, selection: Vec<String>) -> Result<()> {
    bail!("Unhandled select menu '{}'", id)
}

pub async fn handle_modal(ctx: DiscordContext, modal: ModalInteraction) -> Result<()> {
    modal.defer_ephemeral(&ctx.http).await?;

    bail!("Unhandled modal '{}'", modal.data.custom_id)
}

async fn process_interaction_result() {}
