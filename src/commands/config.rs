use crate::{
    Context, Error,
    localization::{get_strings, Language},
};
use std::str::FromStr;

#[poise::command(
    slash_command,
    subcommands("language"),
    required_permissions = "ADMINISTRATOR",
    description_localized("pt-BR", "Comando-pai para todas as configurações do bot."),
    description_localized("en-US", "Parent command for all bot configurations.")
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

#[poise::command(
    slash_command, 
    guild_only = true,
    description_localized("pt-BR", "Define o idioma do bot para este servidor."),
    description_localized("en-US", "Sets the bot's language for this server.")
)]
pub async fn language(
    ctx: Context<'_>,
    #[description_localized("pt-BR", "O idioma a ser usado (pt-br ou en-us)")]
    #[description_localized("en-US", "The language to use (pt-br or en-us)")]
    #[choices("Português (BR)", "English")]
    lang_choice: &str,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

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