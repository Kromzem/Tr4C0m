use crate::{
    api::status::get_status,
    types::{Context, Error},
    util::content_format::{format_field_content, format_field_content_lines_owned},
};

#[poise::command(slash_command, ephemeral)]
pub async fn global_stats(ctx: Context<'_>) -> Result<(), Error> {
    let status = get_status().await?;

    let global_stats_lines = [
        format!("Agents: {}", &status.stats.agents),
        format!("Ships: {}", &status.stats.ships),
        format!("Systems: {}", &status.stats.systems),
        format!("Waypoints: {}", &status.stats.waypoints),
    ];

    let credits_leaderboard = status
        .leaderboards
        .most_credits
        .iter()
        .enumerate()
        .map(|(i, e)| format!("#{} - {} ({})", i, e.agent_symbol, e.credits))
        .collect::<Vec<String>>()
        .join("\n");

    ctx.send(|m| {
        m.embed(|e| {
            e.field(
                "Global stats",
                format_field_content_lines_owned(&global_stats_lines),
                false,
            )
            .field(
                "Leaderboard (Credits)",
                format_field_content(&credits_leaderboard),
                false,
            )
        })
    })
    .await?;

    Ok(())
}
