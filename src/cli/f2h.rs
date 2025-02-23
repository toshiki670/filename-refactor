use crate::cli::Route;

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
        let _input_files = rust_support::glob::expend_glob_input_patterns(&self.input_patterns)?;

        todo!();
    }
}
