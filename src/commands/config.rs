use crate::{
    Context, Error,
    localization::{get_strings, Language},
};
use std::str::FromStr;

/// Comandos para configurar o bot neste servidor
#[poise::command(
    slash_command,
    subcommands("language"),
    required_permissions = "ADMINISTRATOR"
)]
pub async fn config(ctx: Context<'_>) -> Result<(), Error> {
    let lang = if let Some(guild_id) = ctx.guild_id() {
        let map = ctx.data().language_map.read().await;
        map.get(&guild_id).cloned().unwrap_or_default()
    } else {
        Language::default()
    };
    let s = get_strings(lang);
    ctx.say(s.config.use_subcommand).await?;
    Ok(())
}

/// Define o idioma do bot para este servidor
#[poise::command(slash_command, guild_only = true)]
pub async fn language(
    ctx: Context<'_>,
    #[description = "O idioma a ser usado (pt-br ou en-us)"]
    #[choices("Português (BR)", "English")]
    lang_choice: &str, // <-- A MUDANÇA ESTÁ AQUI (de String para &str)
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    // O resto do seu código já funciona perfeitamente com &str!
    let lang = match Language::from_str(lang_choice) {
        Ok(lang) => lang,
        Err(_) => {
            let s = get_strings(Language::default());
            ctx.say(s.config.invalid_lang).await?;
            return Ok(());
        }
    };

    {
        let mut map = ctx.data().language_map.write().await;
        map.insert(guild_id, lang);
    }

    let s = get_strings(lang);
    let response = match lang {
        Language::PtBr => s.config.lang_set_pt,
        Language::EnUs => s.config.lang_set_en,
    };

    ctx.send(
        poise::CreateReply::default()
            .content(response)
            .ephemeral(true),
    )
    .await?;

    Ok(())
}