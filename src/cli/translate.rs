use crate::{cli::Route, transformer::translate::transform_files};

#[derive(clap::Args, Debug)]
#[command(name = "Text Translation")]
pub(super) struct Args {
    #[clap(
        value_name = "INPUT PATTERNS",
        help = "Input files (glob patterns supported: *.json)"
    )]
    input_patterns: Vec<String>,

    #[clap(short, long, value_name = "LANGUAGE", help = "Translate from language")]
    source: Language,

    #[clap(short, long, value_name = "LANGUAGE", help = "Translate to language")]
    target: Language,
}

impl Route for Args {
    async fn route(&self) -> anyhow::Result<()> {
        let input_paths = rust_support::glob::expend_glob_input_patterns(&self.input_patterns)?;

        // 言語に基づいて翻訳関数を選択
        let source = self.source.clone();
        let target = self.target.clone();

        transform_files(input_paths, source.into(), target.into()).await
    }
}

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

impl From<Language> for libretranslate::Language {
    fn from(language: Language) -> Self {
        match language {
            Language::Ja => libretranslate::Language::Japanese,
            Language::En => libretranslate::Language::English,
        }
    }
}
