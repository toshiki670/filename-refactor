use deepl::{DeepLApi, Lang};
use futures::future;

use std::path::PathBuf;

use anyhow::Context as _;

pub async fn transform_files(
    client: &DeepLApi,
    files: Vec<PathBuf>,
    source: &Lang,
    target: &Lang,
) -> anyhow::Result<()> {
    let futures = files
        .into_iter()
        .map(|path| translate(client, path, source, target));

    let results = future::join_all(futures).await;

    let transformed = rust_support::anyhow::collect_results(results)?;

    super::transform_files(transformed).await?;

    Ok(())
}

// Translation functions for different languages
pub async fn translate(
    client: &DeepLApi,
    path: PathBuf,
    source: &Lang,
    target: &Lang,
) -> anyhow::Result<(PathBuf, String)> {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .with_context(|| anyhow::anyhow!("Failed to get filename: {:?}", path))?;

    let response = client
        .translate_text(file_name, target.clone())
        .source_lang(source.clone())
        .await;

    let translated = match response {
        Ok(response) => response
            .translations
            .into_iter()
            .map(|t| t.text)
            .collect::<Vec<String>>()
            .join(""),
        Err(e) => {
            log::error!("Failed to translate: {:?}", e);
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

        if let Err(e) = dotenvy::dotenv() {
            log::error!("Failed to load .env file: {:?}", e);
        }

        let api_key = std::env::var("DEEPL_API_KEY").unwrap();
        let client = DeepLApi::with(&api_key).new();
        let paths = vec![file_path.clone()];
        let source = Lang::JA;
        let target = Lang::EN;
        let result = transform_files(&client, paths, &source, &target).await;

        println!("result: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_translate() {
        if let Err(e) = dotenvy::dotenv() {
            log::error!("Failed to load .env file: {:?}", e);
        }

        let api_key = std::env::var("DEEPL_API_KEY").unwrap();
        let client = DeepLApi::with(&api_key).new();
        let path = PathBuf::from("test.txt");
        let source = Lang::JA;
        let target = Lang::EN;
        let result = translate(&client, path, &source, &target).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_translate_error() {
        let path = PathBuf::from("test.txt");
        let source = Lang::JA;
        let target = Lang::EN;

        let api_key = "invalid_api_key".to_string();
        let client = DeepLApi::with(&api_key).new();
        let result = translate(&client, path, &source, &target).await;

        assert!(result.is_ok());
    }
}
