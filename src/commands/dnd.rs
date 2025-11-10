use crate::{Context, Error, localization::{get_strings, Language}};

/// Rola dados no formato D&D 2014 (ex: 2d20+5)
#[poise::command(slash_command)]
pub async fn dnd2014(
    ctx: Context<'_>,
    #[description = "O dado a rolar (ex: 1d20, 2d6+3)"]
    dice: String,
) -> Result<(), Error> {
    let lang = if let Some(guild_id) = ctx.guild_id() {
        let map = ctx.data().language_map.read().await;
        map.get(&guild_id).cloned().unwrap_or_default()
    } else {
        Language::default()
    };
    let s = get_strings(lang);

    let response = s.dnd.in_development.replace("{}", &dice);
    ctx.say(response).await?;
    Ok(())
}