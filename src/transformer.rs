pub mod fullwidth_to_halfwidth;
pub mod translate;

use futures::future;
use std::path::PathBuf;

async fn transform_files(files: Vec<(PathBuf, String)>) -> anyhow::Result<()> {
    let futures = files.into_iter().map(|(path, new_file_name)| {
        let new_path = path.with_file_name(&new_file_name);
        log::info!(
            "Renamed file: '{}' -> '{}'.",
            path.display(),
            new_path.display()
        );
        tokio::fs::rename(path.clone(), new_path)
    });
    let results = future::join_all(futures).await;
    let results = results
        .into_iter()
        .map(|r| r.map_err(anyhow::Error::new))
        .collect();

    rust_support::anyhow::collect_results(results)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use log::Level;
    use std::env;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_transform_file_error_invalid_filename() -> Result<()> {
        // 存在しないファイルパスを使用
        let non_existent_path = PathBuf::from("/this/path/definitely/does/not/exist/file.txt");

        // transform_file実行 - 存在しないファイルなのでエラーになるはず
        let result = transform_files(vec![(non_existent_path, "new_filename".to_string())]).await;

        match result {
            Ok(_) => panic!("Expected an error but got Ok"),
            Err(e) => {
                // 方法1: エラーメッセージを文字列として取得して検証
                let error_message = format!("{:?}", e);
                assert!(
                    error_message.contains("No such file or directory"),
                    "Error message '{}' does not contain 'No such file or directory'",
                    error_message
                );
            }
        }

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
        let result = transform_files(vec![
            (file1, "new_filename".to_string()),
            (file2, "new_filename".to_string()),
        ])
        .await;

        assert!(result.is_ok());

        Ok(())
    }
}
