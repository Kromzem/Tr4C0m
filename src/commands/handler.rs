use anyhow::Result;
use serenity::{
    all::{CommandInteraction, Message},
    async_trait,
    builder::CreateCommand,
    prelude::Context,
};

#[async_trait]
pub trait CommandHandler {
    fn get_identifier(&self) -> &str;
    fn get_command_builder(&self) -> CreateCommand;
    async fn perform(&self, ctx: &Context, interaction_token: &str) -> Result<()>;
}
