use crate::api::status::get_status;
use crate::types::{Context, Error};
use crate::util::content_format::format_field_content;

#[poise::command(slash_command, ephemeral)]
pub async fn status(ctx: Context<'_>) -> Result<(), Error> {
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
