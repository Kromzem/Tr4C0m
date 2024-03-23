use std::{
    collections::{HashMap, HashSet},
    env,
};

use anyhow::{Context, Result};
use serenity::{
    all::{
        Command, CommandInteraction, ComponentInteraction, GuildId, Interaction, ModalInteraction,
        Ready,
    },
    async_trait,
    builder::{
        CreateCommand, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
        CreateModal, EditInteractionResponse, EditMessage,
    },
    prelude::EventHandler,
};

use crate::{
    command_router,
    // commands::{self, handler::CommandHandler, ping::PingCommandHandler, play::PlayCommandHandler},
    types::DiscordContext,
    util::util::{show_interaction_error, show_message},
    views::view::{handle_view_interaction, View},
};

// type CommandLogic = dyn Fn(DiscordContext) -> Result<()>;

pub struct Handler {
    // commands: HashMap<String, Box<dyn CommandHandler + Send + Sync + 'static>>,
}

impl Handler {
    // pub fn new() -> Handler {
    //     let mut handler = Handler {
    //         commands: HashMap::new(),
    //     };
    //     // handler.add_command(StatusCommandHandler);
    //     handler.add_command(PingCommandHandler);
    //     handler.add_command(PlayCommandHandler);

    //     handler
    // }

    // fn add_command<T: CommandHandler + Send + Sync + 'static>(&mut self, handler: T) {
    //     self.commands
    //         .insert(handler.get_identifier().to_string(), Box::new(handler));
    // }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: DiscordContext, interaction: Interaction) {
        let token = interaction.token().to_string();

        if let Interaction::Command(command) = interaction {
            let result = command_router::handle_command(&ctx, command).await;

            if let Err(error) = result {
                println!("{:?}", error);

                let _ = show_interaction_error(&ctx, &token, &error).await;
            }

            return;
        }

        let result = handle_view_interaction(&ctx, interaction).await;
        if let Err(error) = result {
            println!("{:?}", error);
        }

        // let result = match interaction {
        //     Interaction::Command(command) => command_router::handle_command(&ctx, command).await,
        //     _ => handle_view_interaction(&ctx, interaction).await,
        // };

        // if let Result::Err(err) = result {
        //     println!("{:?}", err);

        //     let message = EditInteractionResponse::new().content(format!(
        //         "```diff\n- {}\n```",
        //         err.to_string().replace("\n", "- \n")
        //     ));

        //     show_message(&ctx, &message, &token).await;
        // }
    }

    async fn ready(&self, ctx: DiscordContext, ready: Ready) {
        command_router::register_commands(ctx)
            .await
            .expect("Setting dev guild commands failed");

        // let dev_guild = read_dev_guild_id();

        // dev_guild
        //     .set_commands(
        //         ctx.http,
        //         self.commands
        //             .values()
        //             .map(|c| c.get_command_builder())
        //             .collect(),
        //     )
        //     .await
        //     .expect("Setting dev guild commands failed");

        println!("Bot started!");
    }
}

impl Handler {
    // async fn handle_command(&self, command: CommandInteraction, ctx: DiscordContext) -> Result<()> {
    //     command.defer_ephemeral(&ctx.http).await?;

    //     if let Err(error) = self.perform_command(&command, &ctx).await {
    //         let message = EditInteractionResponse::new().content(format!(
    //             "```diff\n- {}\n```",
    //             error.to_string().replace("\n", "- \n")
    //         ));

    //         show_message(&ctx, &message, &command.token).await?;
    //     }

    //     Ok(())
    // }

    // async fn perform_command(
    //     &self,
    //     command: &CommandInteraction,
    //     ctx: &serenity::prelude::Context,
    // ) -> Result<()> {
    //     self.commands
    //         .get(&command.data.name)
    //         .context(format!(
    //             "Command handler for '{}' not available",
    //             command.data.name
    //         ))?
    //         .perform(ctx, &command.token)
    //         .await
    // }

    async fn handle_component(
        &self,
        component: ComponentInteraction,
        ctx: &DiscordContext,
    ) -> Result<()> {
        todo!()
    }

    async fn handle_modal(&self, modal: ModalInteraction, ctx: &DiscordContext) -> Result<()> {
        todo!()
    }
}

fn read_dev_guild_id() -> GuildId {
    GuildId::new(
        u64::from_str_radix(&env::var("dev_guild_id").expect("No dev guild id set"), 10)
            .expect("invalid dev guild id"),
    )
}
