use futures::future;
use libretranslate::Language;

use std::path::PathBuf;

use anyhow::Context as _;

pub async fn transform_files(
    files: Vec<PathBuf>,
    source: Language,
    target: Language,
) -> anyhow::Result<()> {
    let futures = files
        .into_iter()
        .map(|path| translate(path, source, target));

    let results = future::join_all(futures).await;

    let transformed = rust_support::anyhow::collect_results(results)?;

    super::transform_files(transformed).await?;

    Ok(())
}

// Translation functions for different languages
pub async fn translate(
    path: PathBuf,
    source: Language,
    target: Language,
) -> anyhow::Result<(PathBuf, String)> {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .with_context(|| anyhow::anyhow!("Failed to get filename: {:?}", path))?;

    let translated = libretranslate::translate(source.into(), target.into(), file_name, None).await;
    let translated = match translated {
        Ok(translated) => translated.output,
        Err(e) => {
            log::error!("Translation failed: {}", e);
            file_name.to_string()
        }
    };
    Ok((path, translated))
}
