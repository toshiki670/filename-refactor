use crate::cli::Route;

#[derive(clap::Args, Debug)]
#[command(name = "Text Translation")]
pub(super) struct Args {
    #[clap(
        value_name = "INPUT PATTERNS",
        help = "Input files (glob patterns supported: *.json)"
    )]
    input_patterns: Vec<String>,

    #[clap(short, long, value_name = "LANGUAGE", help = "Translate to language")]
    language: String,
}

impl Route for Args {
    async fn route(&self) -> anyhow::Result<()> {
        let input_paths = rust_support::glob::expend_glob_input_patterns(&self.input_patterns)?;

        // 言語に基づいて翻訳関数を選択
        let language = self.language.clone();
        let transform_fn = move |s: &str| crate::transformer::translate::translate(s, &language);

        crate::transformer::transform_filenames(&input_paths, transform_fn).await
    }
}
