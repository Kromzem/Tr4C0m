use crate::api::status::get_status;
use crate::util::content_format::format_field_content;
use anyhow::Result;
use serenity::all::CommandInteraction;
use serenity::async_trait;
use serenity::builder::CreateCommand;
use serenity::prelude::Context;

use super::handler::CommandHandler;

pub struct StatusCommandHandler;

const IDENTIFIER: &'static str = "status";

#[async_trait]
impl CommandHandler for StatusCommandHandler {
    fn get_identifier(&self) -> String {
        IDENTIFIER.to_string()
    }

    fn get_command_builder(&self) -> CreateCommand {
        CreateCommand::new(IDENTIFIER).description("Show status information about SpaceTraders")
    }

    async fn perform(&self, ctx: &Context, command: &CommandInteraction) -> Result<()> {
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
}

// pub async fn status(ctx: Context) -> Result<()> {
//     let status = get_status().await?;

//     ctx.send(|m| {
//         m.embed(|e| {
//             e.field("Status", format_field_content(&status.status), false)
//                 .field("Version", format_field_content(&status.version), false)
//                 .field(
//                     "Next server reset",
//                     format_field_content(&status.server_reset.next),
//                     false,
//                 )
//         })
//     })
//     .await?;

//     Ok(())
// }
