use crate::{cli::Route, transformer::fullwidth_to_halfwidth::transform_files};

#[derive(clap::Args, Debug)]
#[command(name = "Fullwidth to Halfwidth")]
pub(super) struct Args {
    #[clap(
        value_name = "INPUT PATTERNS",
        help = "Input files (glob patterns supported: *.json)"
    )]
    input_patterns: Vec<String>,
}

impl Route for Args {
    async fn route(&self) -> anyhow::Result<()> {
        let input_paths = rust_support::glob::expend_glob_input_patterns(&self.input_patterns)?;

        transform_files(input_paths).await
    }
}
