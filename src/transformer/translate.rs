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

    let translated = match std::env::var("TRANSLATE_API_URL") {
        Ok(url) => {
            libretranslate::translate_url(source, target, file_name, &url, None).await
        }
        Err(_) => libretranslate::translate(source, target, file_name, None).await,
    };
    let translated = match translated {
        Ok(translated) => translated.output,
        Err(e) => {
            log::error!("Translation failed: {}", e);
            file_name.to_string()
        }
    };
    Ok((path, translated))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_transform_files() {
        let temp_dir = tempdir().unwrap();
        let file_path = temp_dir.path().join("testfile.txt");

        // ファイルを作成
        tokio::fs::write(&file_path, "test content").await.unwrap();

        let paths = vec![file_path.clone()];
        let source = Language::Japanese;
        let target = Language::English;
        let result = transform_files(paths, source, target).await;

        println!("result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_translate() {
        let path = PathBuf::from("test.txt");
        let source = Language::Japanese;
        let target = Language::English;
        let result = translate(path, source, target).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_translate_error() {
        let path = PathBuf::from("test.txt");
        let source = Language::Japanese;
        let target = Language::English;

        unsafe {
            std::env::set_var("TRANSLATE_API_URL", "http://localhost:1234/");
        }
        let result = translate(path, source, target).await;

        unsafe {
            std::env::remove_var("TRANSLATE_API_URL");
        }
        assert!(result.is_ok());
    }
}
