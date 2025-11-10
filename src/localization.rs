use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Language {
    PtBr,
    EnUs,
}

impl Default for Language {
    fn default() -> Self {
        Language::EnUs
    }
}

impl FromStr for Language {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pt-br" | "pt" | "português" | "português (br)" => Ok(Language::PtBr),
            "en-us" | "en" | "english" => Ok(Language::EnUs),
            _ => Err(()),
        }
    }
}

pub struct DaggerheartStrings {
    pub critical_success: &'static str,
    pub action_success: &'static str,
    pub action_fail: &'static str,
    pub total_is: &'static str,
    pub gain_hope: &'static str,
    pub gain_fear: &'static str,
    pub hope_die: &'static str,
    pub fear_die: &'static str,
    pub modifier: &'static str,
    pub difficulty: &'static str,
    pub roll: &'static str,
    pub stress_heal: &'static str,
    pub extra_damage: &'static str,
    pub adv_dis_field: &'static str,
}

pub struct DndStrings {
    pub total: &'static str,
    pub modifiers: &'static str,
    pub rolls: &'static str,
    pub invalid_format: &'static str,
    pub none: &'static str,
    pub crit_success: &'static str,
    pub crit_failure: &'static str,
}

pub struct RollStrings {
    pub please_choose: &'static str,
}

pub struct HelpStrings {
    pub title: &'static str,
    pub overview: &'static str,
    pub overview_desc: &'static str,
    pub overview_footer: &'static str,
    pub roll_title: &'static str,
    pub roll_desc: &'static str,
    pub roll_daggerheart_desc: &'static str,
    pub roll_dnd2014_desc: &'static str,
    pub config_title: &'static str,
    pub config_desc: &'static str,
    pub config_language_desc: &'static str,
}

pub struct ConfigStrings {
    pub invalid_lang: &'static str,
    pub lang_set_pt: &'static str,
    pub lang_set_en: &'static str,
    pub use_subcommand: &'static str,
    pub lang_choice_pt: &'static str,
    pub lang_choice_en: &'static str,
}

pub struct Strings {
    pub daggerheart: DaggerheartStrings,
    pub dnd: DndStrings,
    pub roll: RollStrings,
    pub config: ConfigStrings,
    pub help: HelpStrings,
}

const PT_STRINGS: Strings = Strings {
    daggerheart: DaggerheartStrings {
        critical_success: "💥 Sucesso Crítico! 💥",
        action_success: "✅ Sucesso!",
        action_fail: "❌ Falha.",
        total_is: "Total:",
        gain_hope: "Você ganha **1 Hope**.",
        gain_fear: "O GM ganha **1 Fear**.",
        hope_die: "Esperança",
        fear_die: "Medo",
        modifier: "⚖️ Modificador",
        difficulty: "🎯 Dificuldade",
        roll: "🎲 Rolagem",
        stress_heal: "Você limpa **1 Stress**.",
        extra_damage: "*(Se for um ataque, causa dano extra)*",
        adv_dis_field: "🎲 Vantagem/Desvantagem (1d6)",
    },
    dnd: DndStrings {
        total: "🎯 Total",
        modifiers: "⚖️ Modificadores",
        rolls: "🎲 Rolagens",
        invalid_format: "Formato de dado inválido! Exemplo: `2d10 + 5`",
        none: "Nenhum",
        crit_success: "💥 Sucesso Crítico!",
        crit_failure: "💀 Falha Crítica!",
    },
    roll: RollStrings {
        please_choose: "Por favor, escolha um sistema (ex: /roll daggerheart ...)",
    },
    config: ConfigStrings {
        invalid_lang: "Idioma inválido. Use 'Português (BR)' ou 'English'.",
        lang_set_pt: "✅ Idioma definido para Português (BR)!",
        lang_set_en: "✅ Language set to English!",
        use_subcommand: "Use um subcomando, como /config language",
        lang_choice_pt: "Português (BR)",
        lang_choice_en: "English",
    },
    help: HelpStrings {
        title: "Ajuda do Bot",
        overview: "Visão Geral dos Comandos",
        overview_desc: "Este bot organiza comandos em grupos. Use `/help <comando>` para mais detalhes.",
        overview_footer: "Exemplo: /help roll",
        roll_title: "Ajuda para: /roll",
        roll_desc: "Comando-pai para todas as rolagens de sistema.",
        roll_daggerheart_desc: "Rola os Duality Dice (Hope/Fear) de Daggerheart.",
        roll_dnd2014_desc: "Rola dados no formato D&D (ex: 2d20+5, 2>d20).",
        config_title: "Ajuda para: /config",
        config_desc: "Comando-pai para todas as configurações do bot.",
        config_language_desc: "Define o idioma do bot para este servidor.",
    },
};

const EN_STRINGS: Strings = Strings {
    daggerheart: DaggerheartStrings {
        critical_success: "💥 Critical Success! 💥",
        action_success: "✅ Success!",
        action_fail: "❌ Failure.",
        total_is: "Total:",
        gain_hope: "You gain **1 Hope**.",
        gain_fear: "The GM gains **1 Fear**.",
        hope_die: "Hope",
        fear_die: "Fear",
        modifier: "⚖️ Modifier",
        difficulty: "🎯 Difficulty",
        roll: "🎲 Roll",
        stress_heal: "You clear **1 Stress**.",
        extra_damage: "*(If this is an attack, it deals extra damage)*",
        adv_dis_field: "🎲 Advantage/Disadvantage (1d6)",
    },
    dnd: DndStrings {
        total: "🎯 Total",
        modifiers: "⚖️ Modifiers",
        rolls: "🎲 Rolls",
        invalid_format: "Invalid dice format! Example: `2d10 + 5`",
        none: "None",
        crit_success: "💥 Critical Success!",
        crit_failure: "💀 Critical Failure!",
    },
    roll: RollStrings {
        please_choose: "Please choose a system (e.g., /roll daggerheart ...)",
    },
    config: ConfigStrings {
        invalid_lang: "Invalid language. Use 'Português (BR)' or 'English'.",
        lang_set_pt: "✅ Idioma definido para Português (BR)!",
        lang_set_en: "✅ Language set to English!",
        use_subcommand: "Use a subcommand, like /config language",
        lang_choice_pt: "Português (BR)",
        lang_choice_en: "English",
    },
    help: HelpStrings {
        title: "Bot Help",
        overview: "Commands Overview",
        overview_desc: "This bot organizes commands into groups. Use `/help <command>` for more details.",
        overview_footer: "Example: /help roll",
        roll_title: "Help for: /roll",
        roll_desc: "Parent command for all system rolls.",
        roll_daggerheart_desc: "Rolls the Duality Dice (Hope/Fear) for Daggerheart.",
        roll_dnd2014_desc: "Rolls D&D-style dice (e.g., 2d20+5, 2>d20).",
        config_title: "Help for: /config",
        config_desc: "Parent command for all bot configurations.",
        config_language_desc: "Sets the bot's language for this server.",
    },
};

pub fn get_strings(lang: Language) -> &'static Strings {
    match lang {
        Language::PtBr => &PT_STRINGS,
        Language::EnUs => &EN_STRINGS,
    }
}