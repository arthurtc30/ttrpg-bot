use crate::{Context, Error, localization::{get_strings, Language}};

#[poise::command(
    slash_command,
    subcommands("super::daggerheart::daggerheart", "super::dnd::dnd2014")
)]
pub async fn roll(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let lang = if let Some(guild_id) = ctx.guild_id() {
        let map = ctx.data().language_map.read().await;
        map.get(&guild_id).cloned().unwrap_or_default()
    } else {
        Language::default()
    };
    let s = get_strings(lang);

    ctx.say(s.roll.please_choose).await?;
    Ok(())
}