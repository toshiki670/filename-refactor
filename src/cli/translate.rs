use anyhow::Context as _;
use clap::{ValueEnum, builder::PossibleValue};
use deepl::{DeepLApi, Lang};

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

        let source = self.source;
        let target = self.target;

        let api_key = match std::env::var("DEEPL_API_KEY") {
            Ok(key) => key,
            Err(_) => recive_api_key_with_interactive()?,
        };

        let client = DeepLApi::with(&api_key).new();

        transform_files(&client, input_paths, &source.into(), &target.into()).await
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    Ja,
    En,
    Ar,
    De,
    Es,
    Fr,
    It,
    Pt,
    Ru,
    Zh,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Ja => write!(f, "ja"),
            Language::En => write!(f, "en"),
            Language::Ar => write!(f, "ar"),
            Language::De => write!(f, "de"),
            Language::Es => write!(f, "es"),
            Language::Fr => write!(f, "fr"),
            Language::It => write!(f, "it"),
            Language::Pt => write!(f, "pt"),
            Language::Ru => write!(f, "ru"),
            Language::Zh => write!(f, "zh"),
        }
    }
}

impl std::str::FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ja" => Ok(Language::Ja),
            "en" => Ok(Language::En),
            "ar" => Ok(Language::Ar),
            "de" => Ok(Language::De),
            "es" => Ok(Language::Es),
            "fr" => Ok(Language::Fr),
            "it" => Ok(Language::It),
            "pt" => Ok(Language::Pt),
            "ru" => Ok(Language::Ru),
            "zh" => Ok(Language::Zh),
            _ => Err(format!("Invalid language: {}", s)),
        }
    }
}

impl ValueEnum for Language {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Language::Ja,
            Language::En,
            Language::Ar,
            Language::De,
            Language::Es,
            Language::Fr,
            Language::It,
            Language::Pt,
            Language::Ru,
            Language::Zh,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Language::Ja => PossibleValue::new("ja"),
            Language::En => PossibleValue::new("en"),
            Language::Ar => PossibleValue::new("ar"),
            Language::De => PossibleValue::new("de"),
            Language::Es => PossibleValue::new("es"),
            Language::Fr => PossibleValue::new("fr"),
            Language::It => PossibleValue::new("it"),
            Language::Pt => PossibleValue::new("pt"),
            Language::Ru => PossibleValue::new("ru"),
            Language::Zh => PossibleValue::new("zh"),
        })
    }
}

impl From<Language> for Lang {
    fn from(language: Language) -> Self {
        match language {
            Language::Ja => Lang::JA,
            Language::En => Lang::EN,
            Language::Ar => Lang::AR,
            Language::De => Lang::DE,
            Language::Es => Lang::ES,
            Language::Fr => Lang::FR,
            Language::It => Lang::IT,
            Language::Pt => Lang::PT,
            Language::Ru => Lang::RU,
            Language::Zh => Lang::ZH,
        }
    }
}

fn recive_api_key_with_interactive() -> anyhow::Result<String> {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();

    let api_key = rpassword::prompt_password_from_bufread(
        &mut stdin_lock,
        &mut stdout_lock,
        "Enter your DeepL API key: ",
    )
    .context("Failed to read API key")?;

    Ok(api_key)
}
