use rand::Rng;
use poise::{CreateReply, ChoiceParameter};
use poise::serenity_prelude as serenity;
use crate::{Context, Error, localization::{get_strings, Language}};

#[derive(Debug, ChoiceParameter)]
pub enum AdvantageState {
    #[name = "Vantagem (+1d6)"]
    #[name_localized("en-US", "Advantage (+1d6)")]
    Advantage,
    #[name = "Desvantagem (-1d6)"]
    #[name_localized("en-US", "Disadvantage (-1d6)")]
    Disadvantage,
}

#[poise::command(
    slash_command,
    description_localized("pt-BR", "Rola os Duality Dice (Hope/Fear) de Daggerheart."),
    description_localized("en-US", "Rolls the Duality Dice (Hope/Fear) for Daggerheart.")
)]
pub async fn daggerheart(
    ctx: Context<'_>,
    #[description_localized("pt-BR", "Modificador a ser somado (ex: 3, -1)")]
    #[description_localized("en-US", "Modifier to add (e.g., 3, -1)")]
    modifier: i32,

    #[description_localized("pt-BR", "Dificuldade (opcional)")]
    #[description_localized("en-US", "Difficulty (optional)")]
    difficulty: Option<i32>,

    #[description_localized("pt-BR", "Rolar com Vantagem (+1d6) ou Desvantagem (-1d6)")]
    #[description_localized("en-US", "Roll with Advantage (+1d6) or Disadvantage (-1d6)")]
    adv_dis: Option<AdvantageState>,

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

    let mut adv_dis_roll = 0_i32;
    let mut adv_dis_str = String::new();

    if let Some(choice) = adv_dis {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(1..=6);
        
        match choice {
            AdvantageState::Advantage => {
                adv_dis_roll = roll;
                adv_dis_str = format!("{:+}", roll);
            }
            AdvantageState::Disadvantage => {
                adv_dis_roll = -roll;
                adv_dis_str = format!("{}", -roll);
            }
        }
    }

    let total_sum = hope_die + fear_die + modifier + adv_dis_roll;
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

    if adv_dis_roll != 0 {
        embed = embed.field(ds.adv_dis_field, adv_dis_str, false);
    }

    embed = embed.field(ds.modifier, modifier_str, false);
    embed = embed.field(ds.roll, dice_roll_str, false);

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}