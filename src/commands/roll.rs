use crate::{Context, Error, localization::{get_strings, Language}};

#[poise::command(
    slash_command,
    description_localized("pt-BR", "Comando-pai para todas as rolagens de sistema"),
    description_localized("en-US", "Parent command for all system rolls"),
    subcommands("super::daggerheart::dh", "super::default::dice")
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