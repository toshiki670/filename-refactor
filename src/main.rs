mod cli;
mod fullwidth_to_halfwidth;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::Args::run().await?;
    Ok(())
}
