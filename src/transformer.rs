pub mod fullwidth_to_halfwidth;
pub mod translate;

use futures::future;
use std::path::PathBuf;

// 共通のファイル変換処理
pub(crate) async fn transform_filenames<F>(paths: &[PathBuf], transform_fn: F) -> anyhow::Result<()>
where
    F: Fn(&str) -> String + Clone + Send + Sync + 'static,
{
    let futures = paths.iter().map(move |path| {
        let transform_fn = transform_fn.clone();
        transform_file(path, transform_fn)
    });
    let results = future::join_all(futures).await;
    rust_support::anyhow::collect_results(results)?;
    Ok(())
}

async fn transform_file<F>(path: &PathBuf, transform_fn: F) -> anyhow::Result<()>
where
    F: Fn(&str) -> String,
{
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to get filename: {:?}", path))?;

    let new_file_name = transform_fn(file_name);

    if new_file_name != file_name {
        let new_path = path.with_file_name(&new_file_name);
        tokio::fs::rename(path, &new_path).await?;
        log::info!(
            "Renamed file: '{}' -> '{}'.",
            file_name,
            new_file_name
        );
    } else {
        log::info!("File '{}' is already transformed.", file_name);
    }
    Ok(())
}
