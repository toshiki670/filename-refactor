
#[derive(Debug, Clone, Copy)]
pub enum Language {
    Ja,
    En,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Ja => write!(f, "ja"),
            Language::En => write!(f, "en"),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ja" => Ok(Language::Ja),
            "en" => Ok(Language::En),
            _ => Err(format!("Invalid language: {}", s)),
        }
    }
}



// Translation functions for different languages
pub(crate) fn translate(filename: &str, language: Language) -> String {
    match language {
        Language::Ja => translate_to_japanese(filename),
        Language::En => translate_to_english(filename),
    }
}

fn translate_to_japanese(filename: &str) -> String {
    // 日本語への翻訳ロジック（例示のみ）
    format!("ja_{}", filename)
}

fn translate_to_english(filename: &str) -> String {
    // 英語への翻訳ロジック
    format!("en_{}", filename)
}
