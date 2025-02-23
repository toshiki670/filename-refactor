use std::path::PathBuf;

use futures::future;

pub(crate) async fn convert(paths: &[PathBuf]) -> anyhow::Result<()> {
    let futures = paths.iter().map(convert_file);
    let results = future::join_all(futures).await;
    rust_support::anyhow::collect_results(results)?;
    Ok(())
}

async fn convert_file(path: &PathBuf) -> anyhow::Result<()> {
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Failed to retrieve file name: {:?}.", path))?;
    let new_file_name = string_full2half(file_name);
    if new_file_name != file_name {
        let new_path = path.with_file_name(&new_file_name);
        tokio::fs::rename(path, &new_path).await?;
        log::info!("Renamed file: '{}' -> '{}'.", file_name, new_file_name);
    } else {
        log::info!("File name '{}' is already in halfwidth.", file_name);
    }
    Ok(())
}

fn string_full2half(s: &str) -> String {
    s.chars().map(full2half).collect()
}

fn full2half(c: char) -> char {
    match c {
        // Don't convert '／'
        '\u{FF0F}' => c,
        // half ascii code
        '\u{0020}'..='\u{007E}' => c,
        // FullWidth
        // '！'..='～' = '\u{FF01}'..='\u{FF5E}'
        '\u{FF01}'..='\u{FF5E}' => char_from_u32(c as u32 - 0xFF01 + 0x21, c),
        // space
        '\u{2002}'..='\u{200B}' => ' ',
        '\u{3000}' | '\u{FEFF}' => ' ',
        // others
        _ => c,
    }
}

fn char_from_u32(i: u32, def: char) -> char {
    char::from_u32(i).unwrap_or(def)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn test_convert_file() {
        let temp_dir = tempfile::tempdir().unwrap();

        let path1 = temp_dir
            .path()
            .join("ｔｅｓｔ１２３あいうえおhalfwidth漢字.txt");
        tokio::fs::write(&path1, "").await.unwrap();

        let path2 = temp_dir
            .path()
            .join("ｔｅｓｔ１２３あいうえおhalfwidth漢字2.txt");
        tokio::fs::write(&path2, "").await.unwrap();

        let path3 = temp_dir.path().join("abc.txt");
        tokio::fs::write(&path3, "").await.unwrap();

        let paths = vec![path1, path2, path3];

        // convert
        convert(&paths).await.unwrap();

        // check result
        let actual_path1 = temp_dir.path().join("test123あいうえおhalfwidth漢字.txt");
        let actual_path2 = temp_dir.path().join("test123あいうえおhalfwidth漢字2.txt");
        let actual_path3 = temp_dir.path().join("abc.txt");
        assert!(actual_path1.exists());
        assert!(actual_path2.exists());
        assert!(actual_path3.exists());
    }

    #[tokio::test]
    async fn test_string_full2half() {
        assert_eq!(
            string_full2half("ｔｅｓｔ１２３あいうえおhalfwidth漢字.txt"),
            "test123あいうえおhalfwidth漢字.txt"
        );
    }

    #[tokio::test]
    async fn not_convert() {
        assert_eq!(full2half('／'), '／');
    }
}
