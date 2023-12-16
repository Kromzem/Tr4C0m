use crate::{
    api::{agent::get_agent, systems::get_waypoint},
    types::ApplicationContext,
};
use anyhow::Result;

use super::checks::get_user_session;

#[poise::command(slash_command, ephemeral)]
pub async fn show_headquarter_waypoint(ctx: ApplicationContext<'_>) -> Result<()> {
    ctx.defer_ephemeral().await?;

    let token = get_user_session(ctx).await?;
    let agent = get_agent(&token).await?.data;
    let waypoint = get_waypoint(&agent.headquarters).await?.data;

    let orbital_symbols: Vec<String> = waypoint
        .orbitals
        .iter()
        .map(|x| format!("- {}", x.symbol))
        .collect();
    let orbitals_text = orbital_symbols.join("\n");

    ctx.send(|m| {
        m.embed(|e| {
            e.description("Waypoint data to your factions headquarter:")
                .field("Waypoint", &waypoint.symbol, true)
                .field("System", &waypoint.system_symbol, true)
                .field("Type", &waypoint.waypoint_type, true)
                .field("Orbitals", orbitals_text, false)
                .field(
                    "Orbits",
                    &waypoint.orbits.unwrap_or("None".to_string()),
                    false,
                )
                .field(
                    "Coords",
                    format!("X/Y={}/{}", waypoint.x, waypoint.y),
                    false,
                )
        })
    })
    .await?;

    Ok(())
}
