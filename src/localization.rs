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
}

pub struct DndStrings {
    pub in_development: &'static str,
}

pub struct RollStrings {
    pub please_choose: &'static str,
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
    },
    dnd: DndStrings {
        in_development: "(Em desenvolvimento) Rolando: {}",
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
    },
    dnd: DndStrings {
        in_development: "(In Development) Rolling: {}",
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
};

pub fn get_strings(lang: Language) -> &'static Strings {
    match lang {
        Language::PtBr => &PT_STRINGS,
        Language::EnUs => &EN_STRINGS,
    }
}