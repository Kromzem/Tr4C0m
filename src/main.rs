mod api;
mod commands;
mod event_handler;
// mod fragment;
mod command_router;
mod state;
mod types;
mod util;
mod views;

use std::env;

// use commands::status::StatusCommandHandler;
use dotenv::dotenv;
use event_handler::Handler;
// use poise::{builtins, serenity_prelude::GuildId};
use serenity::{
    all::{GuildId, UserId},
    client::ClientBuilder,
    prelude::GatewayIntents,
    Client,
};
// use types::Data;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut client = Client::builder(
        env::var("token").expect("Missing bot token"),
        GatewayIntents::empty(),
    )
    .event_handler(Handler {})
    .await
    .expect("Creating client failed");

    if let Err(err) = client.start().await {
        println!("Start failed: {:?}", err);
    }

    // let framework = poise::Framework::builder()
    //     .options(poise::FrameworkOptions {
    //         commands: vec![
    //             commands::status::status(), //commands::factions::factions(),
    //             commands::stats::global_stats(),
    //             commands::register::register(),
    //             commands::login::login(),
    //             commands::logout::logout(),
    //             commands::me::me(),
    //             commands::show_headquarters_waypoint::show_headquarter_waypoint(),
    //         ],
    //         ..Default::default()
    //     })
    //     .token(env::var("token").expect("missing bot token"))
    //     .intents(poise::serenity_prelude::GatewayIntents::non_privileged())
    //     .setup(|ctx, _ready, framework| {
    //         Box::pin(async move {
    //             builtins::register_in_guild(
    //                 ctx,
    //                 &framework.options().commands,
    //                 read_dev_guild_id(),
    //             )
    //             .await?;
    //             Ok(Data::new())
    //         })
    //     });

    // framework.run().await.expect("could not start bot");
}
