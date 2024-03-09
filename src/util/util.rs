use anyhow::{bail, Result};
use serenity::{all::Message, builder::EditInteractionResponse, prelude::Context};

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
