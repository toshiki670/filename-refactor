mod cli;
mod transformer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::Args::run().await?;
    Ok(())
}
