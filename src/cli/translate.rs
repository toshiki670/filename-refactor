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
            Err(_) => recive_api_key_with_interactive(),
        };

        let client = DeepLApi::with(&api_key).new();

        transform_files(&client, input_paths, &source.into(), &target.into()).await
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

impl ValueEnum for Language {
    fn value_variants<'a>() -> &'a [Self] {
        &[Language::Ja, Language::En]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Language::Ja => PossibleValue::new("ja"),
            Language::En => PossibleValue::new("en"),
        })
    }
}

impl From<Language> for Lang {
    fn from(language: Language) -> Self {
        match language {
            Language::Ja => Lang::JA,
            Language::En => Lang::EN,
        }
    }
}

fn recive_api_key_with_interactive() -> String {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let stdout = std::io::stdout();
    let mut stdout_lock = stdout.lock();

    let api_key = rpassword::prompt_password_from_bufread(
        &mut stdin_lock,
        &mut stdout_lock,
        "Enter your DeepL API key: ",
    )
    .unwrap();
    api_key
}
