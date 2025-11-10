use rand::Rng;
use poise::CreateReply;
use poise::serenity_prelude as serenity;
use crate::{Context, Error, localization::{get_strings, Language}};

/// Rola dados no formato Daggerheart
#[poise::command(slash_command)]
pub async fn daggerheart(
    ctx: Context<'_>,
    #[description = "Modificador a ser somado (ex: 3, -1)"]
    modifier: i32,
    #[description = "Dificuldade (opcional)"]
    difficulty: Option<i32>,
) -> Result<(), Error> {

    let lang = if let Some(guild_id) = ctx.guild_id() {
        let map = ctx.data().language_map.read().await;
        map.get(&guild_id).cloned().unwrap_or_default()
    } else {
        Language::default()
    };
    let s = get_strings(lang);
    let ds = &s.daggerheart;

    let (hope_die, fear_die) = {
        let mut rng = rand::thread_rng();
        (rng.gen_range(1..=12), rng.gen_range(1..=12))
    };

    let total_sum = hope_die + fear_die + modifier;
    let title: String;
    let narrative_result: String;
    let color: u32;

    if hope_die == fear_die {
        title = ds.critical_success.to_string();
        narrative_result = format!(
            "**{}** (independente da Dificuldade)\n{}\n{}\n{}",
            ds.action_success,
            ds.gain_hope,
            ds.stress_heal,
            ds.extra_damage
        );
        color = 0xFFD700;

    } else {
        if let Some(diff) = difficulty {
             if total_sum >= diff {
                title = format!("{} ({} {})", ds.action_success, ds.total_is, total_sum);
             } else {
                title = format!("{} ({} {})", ds.action_fail, ds.total_is, total_sum);
             }
        } else {
            title = format!("{} {}", ds.total_is, total_sum);
        }

        if hope_die > fear_die {
            narrative_result = ds.gain_hope.to_string();
            color = 0xF5CA31;
        } else {
            narrative_result = ds.gain_fear.to_string();
            color = 0x9631F5;
        }
    }

    let dice_roll_str = format!(
        "**{}:** {}\n**{}:** {}", 
        ds.hope_die, hope_die,
        ds.fear_die, fear_die
    );

    let modifier_str = format!("{:+}", modifier);
    let mut embed = serenity::CreateEmbed::default()
        .title(title)
        .description(narrative_result)
        .color(color);

    if let Some(diff) = difficulty {
        embed = embed.field(ds.difficulty, diff.to_string(), false);
    }

    embed = embed.field(ds.modifier, modifier_str, false);
    embed = embed.field(ds.roll, dice_roll_str, false);

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}