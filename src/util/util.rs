use std::collections::HashMap;

use anyhow::{bail, Error, Result};
use serenity::{
    all::{ActionRowComponent, Message, ModalInteractionData},
    builder::{CreateActionRow, CreateInputText, CreateModal, EditInteractionResponse},
    prelude::Context,
};

use crate::{state::get_view_token, types::DiscordContext};

pub fn format_field_content_lines_owned(lines: &[String]) -> String {
    format_field_content(&lines.join("\n"))
}

pub fn format_field_content_lines(lines: &[&str]) -> String {
    format_field_content(&lines.join("\n"))
}

pub fn format_field_content(content: &str) -> String {
    format!("```diff\n{}\n```", content)
}

pub async fn show_message(
    ctx: &DiscordContext,
    message: &EditInteractionResponse,
    interaction_token: &str,
) -> Result<()> {
    ctx.http
        .edit_original_interaction_response(interaction_token, message, Vec::new())
        .await?;

    Ok(())
}

pub async fn show_message_for_user(
    ctx: &DiscordContext,
    message: &EditInteractionResponse,
    user_id: u64,
) -> Result<()> {
    let Some(token) = get_view_token(user_id) else {
        bail!("No active view found for this user!")
    };

    show_message(ctx, message, &token).await
}

pub async fn show_interaction_result(
    ctx: &DiscordContext,
    interaction_token: &str,
    result: Result<()>,
) -> Result<()> {
    if let Err(error) = result {
        let message = EditInteractionResponse::new().content(format!(
            "```diff\n- {}\n```",
            error.to_string().replace("\n", "- \n")
        ));

        return show_message(ctx, &message, interaction_token).await;
    }

    Ok(())
}

pub async fn show_interaction_error(
    ctx: &DiscordContext,
    interaction_token: &str,
    error: &Error,
) -> Result<()> {
    let message = EditInteractionResponse::new().content(format!(
        "```diff\n- {}\n```",
        error.to_string().replace("\n", "- \n")
    ));

    return show_message(ctx, &message, interaction_token).await;
}

pub fn create_modal(custom_id: &str, title: &str, inputs: Vec<CreateInputText>) -> CreateModal {
    CreateModal::new(custom_id, title).components(
        inputs
            .into_iter()
            .map(|i| CreateActionRow::InputText(i))
            .collect(),
    )
}
