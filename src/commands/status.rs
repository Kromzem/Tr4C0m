use crate::api::status::get_status;
use crate::types::ApplicationContext;
use crate::util::content_format::format_field_content;
use anyhow::Result;

#[poise::command(slash_command, ephemeral)]
pub async fn status(ctx: ApplicationContext<'_>) -> Result<()> {
    let status = get_status().await?;

    ctx.send(|m| {
        m.embed(|e| {
            e.field("Status", format_field_content(&status.status), false)
                .field("Version", format_field_content(&status.version), false)
                .field(
                    "Next server reset",
                    format_field_content(&status.server_reset.next),
                    false,
                )
        })
    })
    .await?;

    Ok(())
}
