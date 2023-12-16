mod api;
mod commands;
mod types;
mod util;

use std::env;

use dotenv::dotenv;
use poise::{builtins, serenity_prelude::GuildId};
use types::Data;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::status::status(), //commands::factions::factions(),
                commands::stats::global_stats(),
                commands::register::register(),
                commands::login::login(),
                commands::logout::logout(),
                commands::me::me(),
                commands::show_headquarters_waypoint::show_headquarter_waypoint(),
            ],
            ..Default::default()
        })
        .token(env::var("token").expect("missing bot token"))
        .intents(poise::serenity_prelude::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    read_dev_guild_id(),
                )
                .await?;
                Ok(Data::new())
            })
        });

    framework.run().await.expect("could not start bot");
}

fn read_dev_guild_id() -> GuildId {
    GuildId(
        u64::from_str_radix(&env::var("dev_guild_id").expect("No dev guild id set"), 10)
            .expect("invalid dev guild id"),
    )
}
