pub mod fullwidth_to_halfwidth;
pub mod translate;

use futures::future;
use std::path::PathBuf;

// Common file transformation processing
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
        log::info!("Renamed file: '{}' -> '{}'.", file_name, new_file_name);
    } else {
        log::info!("File '{}' is already transformed.", file_name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use log::Level;
    use pretty_assertions::assert_eq;
    use std::env;
    use tempfile::tempdir;

    // Mock transformation function (converts lowercase to uppercase)
    fn mock_transform(s: &str) -> String {
        s.to_uppercase()
    }

    #[tokio::test]
    async fn test_transform_file_error_invalid_filename() -> Result<()> {
        // 存在しないファイルパスを使用
        let non_existent_path = PathBuf::from("/this/path/definitely/does/not/exist/file.txt");

        // transform_file実行 - 存在しないファイルなのでエラーになるはず
        let result = transform_file(&non_existent_path, mock_transform).await;

        if let Err(e) = &result {
            match e.downcast_ref::<std::io::Error>() {
                Some(io_error) => {
                    assert_eq!(io_error.kind(), std::io::ErrorKind::NotFound);
                }
                None => {
                    panic!("Expected std::io::Error, got {:?}", e);
                }
            }
        }

        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_transform_file_rename_error() -> Result<()> {
        unsafe {
            env::set_var("RUST_LOG", Level::Trace.to_string());
        }

        // Test case where a filename transformation error occurs (e.g., when a file with the same name already exists)
        let temp_dir = tempdir()?;

        // Create two files with the same content
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        tokio::fs::write(&file1, "content1").await?;
        tokio::fs::write(&file2, "content2").await?;

        // Function that converts both files to the same name
        let always_same = |s: &str| s.to_string();

        // Convert the first file
        let result = transform_filenames(&[file1, file2], always_same).await;

        assert!(result.is_ok());

        Ok(())
    }
}
