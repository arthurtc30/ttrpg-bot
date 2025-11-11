use crate::{Context, Error, localization::{get_strings, Language}};
use poise::serenity_prelude as serenity;

async fn get_lang(ctx: &Context<'_>) -> Language {
    if let Some(guild_id) = ctx.guild_id() {
        let map = ctx.data().language_map.read().await;
        map.get(&guild_id).cloned().unwrap_or_default()
    } else {
        Language::default()
    }
}

async fn show_help_overview(ctx: Context<'_>) -> Result<(), Error> {
    let s = &get_strings(get_lang(&ctx).await).help;
    let embed = serenity::CreateEmbed::default()
        .title(s.title)
        .description(s.overview_desc)
        .field(s.overview, "/roll\n/config\n/help", false)
        .footer(serenity::CreateEmbedFooter::new(s.overview_footer))
        .color(0x5865F2);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

async fn show_roll_help(ctx: Context<'_>) -> Result<(), Error> {
    let s = &get_strings(get_lang(&ctx).await).help;
    let embed = serenity::CreateEmbed::default()
        .title(s.roll_title)
        .description(s.roll_desc)
        .field("/roll dh", s.roll_dh_desc, false)
        .field("/roll dice", s.roll_dice_desc, false)
        .color(0x5865F2);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

async fn show_config_help(ctx: Context<'_>) -> Result<(), Error> {
    let s = &get_strings(get_lang(&ctx).await).help;
    let embed = serenity::CreateEmbed::default()
        .title(s.config_title)
        .description(s.config_desc)
        .field("/config language", s.config_language_desc, false)
        .color(0x5865F2);
    ctx.send(poise::CreateReply::default().embed(embed)).await?;
    Ok(())
}

#[poise::command(
    slash_command,
    subcommands("roll", "config"),
    description_localized("pt-BR", "Exibe o menu de ajuda do bot"),
    description_localized("en-US", "Displays the bot's help menu")
)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    show_help_overview(ctx).await
}

#[poise::command(
    slash_command,
    rename = "roll",
    description_localized("pt-BR", "Exibe ajuda para os comandos /roll"),
    description_localized("en-US", "Displays help for the /roll commands")
)]
pub async fn roll(ctx: Context<'_>) -> Result<(), Error> {
    show_roll_help(ctx).await
}

#[poise::command(
    slash_command,
    rename = "config",
    description_localized("pt-BR", "Exibe ajuda para os comandos /config"),
    description_localized("en-US", "Displays help for the /config commands")
)]
pub async fn config(ctx: Context<'_>) -> Result<(), Error> {
    show_config_help(ctx).await
}