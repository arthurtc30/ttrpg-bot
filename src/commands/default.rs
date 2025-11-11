use crate::{Context, Error, localization::{get_strings, Language}};
use rand::Rng;
use poise::CreateReply;
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    description_localized("pt-BR", "Rola dados no formato padrão (ex: 2d20+5, 2>d20)."),
    description_localized("en-US", "Rolls default  dice (e.g., 2d20+5, 2>d20).")
)]
pub async fn dice(
    ctx: Context<'_>,
    #[description_localized("pt-BR", "O dado a rolar (ex: 1d20, 2>d20-1, 2<d10 + 2)")]
    #[description_localized("en-US", "The dice to roll (e.g., 1d20, 2>d20-1, 2<d10 + 2)")]
    dice: String,
) -> Result<(), Error> {
    let lang = if let Some(guild_id) = ctx.guild_id() {
        let map = ctx.data().language_map.read().await;
        map.get(&guild_id).cloned().unwrap_or_default()
    } else {
        Language::default()
    };
    let s = get_strings(lang);
    let ds = &s.dice;

    let (title_str, modifiers_str, rolls_str, is_crit, is_fumble) = {
        #[derive(Debug, PartialEq, Eq)]
        enum RollType {
            Normal,
            Advantage,
            Disadvantage,
        }

        let mut total = 0_i32;
        let mut rolls_list: Vec<String> = Vec::new();
        let mut modifiers_list: Vec<i32> = Vec::new();

        let mut rng = ctx.data().rng.lock().await;

        let mut is_crit = false;
        let mut is_fumble = false;

        let processed_dice = dice
            .replace(' ', "")
            .replace('+', " +")
            .replace('-', " -");

        let parts = processed_dice.split_whitespace();

        for part in parts {
            if part.is_empty() { continue; }

            if part.contains('d') || part.contains('D') {
                let is_negative = part.starts_with('-');
                let part_no_sign = part.trim_start_matches(|c| c == '+' || c == '-');

                let d_index = match part_no_sign.find(['d', 'D']) {
                    Some(i) => i,
                    None => return Err(Error::msg(ds.invalid_format)),
                };

                let (prefix_str, num_sides_str) = part_no_sign.split_at(d_index);
                let num_sides_str = &num_sides_str[1..];

                let num_sides = match num_sides_str.parse::<i32>() {
                    Ok(n) if n > 0 => n,
                    _ => { return Err(Error::msg(ds.invalid_format)); }
                };

                let (num_dice_str, roll_type) = 
                    if let Some(stripped) = prefix_str.strip_suffix('>') {
                        (stripped, RollType::Advantage)
                    } else if let Some(stripped) = prefix_str.strip_suffix('<') {
                        (stripped, RollType::Disadvantage)
                    } else {
                        (prefix_str, RollType::Normal)
                    };

                let num_dice_str = if num_dice_str.is_empty() { "1" } else { num_dice_str };
                
                let num_dice = match num_dice_str.parse::<i32>() {
                    Ok(n) if n > 0 => n,
                    _ => { return Err(Error::msg(ds.invalid_format)); }
                };

                let mut current_rolls: Vec<i32> = Vec::new();
                for _ in 0..num_dice {
                    let roll = rng.gen_range(1..=num_sides);
                    current_rolls.push(roll);
                    rolls_list.push(format!("- {} (d{})", roll, num_sides));

                    if num_sides == 20 {
                        if roll == 20 { is_crit = true; }
                        if roll == 1 { is_fumble = true; }
                    }
                }

                let value_to_add = match roll_type {
                    RollType::Normal => current_rolls.iter().sum(),
                    RollType::Advantage => *current_rolls.iter().max().unwrap_or(&0),
                    RollType::Disadvantage => *current_rolls.iter().min().unwrap_or(&0),
                };

                if is_negative {
                    total = total.saturating_sub(value_to_add);
                } else {
                    total = total.saturating_add(value_to_add);
                }

            } else {
                let modifier = match part.parse::<i32>() {
                    Ok(n) => n,
                    _ => { return Err(Error::msg(ds.invalid_format)); }
                };
                modifiers_list.push(modifier);
                total = total.saturating_add(modifier);
            }
        }

        let modifiers_str = if modifiers_list.is_empty() {
            ds.none.to_string()
        } else {
            modifiers_list.iter()
                .map(|m| format!("{:+}", m))
                .collect::<Vec<String>>()
                .join(", ")
        };

        let rolls_str = if rolls_list.is_empty() {
            ds.none.to_string()
        } else {
            rolls_list.join("\n")
        };

        let title_str = format!("{} {}", ds.total, total);

        (title_str, modifiers_str, rolls_str, is_crit, is_fumble)
    };

    let mut final_title = title_str;

    if is_crit {
        final_title = format!("{} ({})", ds.crit_success, final_title);
    } else if is_fumble {
        final_title = format!("{} ({})", ds.crit_failure, final_title);
    }

    let embed = serenity::CreateEmbed::default()
        .title(final_title)
        .color(0x5865F2) 
        .field(ds.modifiers, modifiers_str, false)
        .field(ds.rolls, rolls_str, false);

    match ctx.send(CreateReply::default().embed(embed)).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}