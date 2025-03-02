pub fn string_full2half(s: &str) -> String {
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

        // CLIと同じ方法で直接transform_filenamesを呼び出す
        crate::transformer::transform_filenames(&paths, string_full2half)
            .await
            .unwrap();

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
        assert_eq!(string_full2half("／"), "／");
    }

    #[test]
    fn test_full2half_conversions() {
        // 全角から半角への変換テスト

        // 1. '／' はそのまま
        assert_eq!(full2half('／'), '／');

        // 2. 半角ASCII文字はそのまま
        assert_eq!(full2half('A'), 'A');
        assert_eq!(full2half('a'), 'a');
        assert_eq!(full2half('1'), '1');
        assert_eq!(full2half(' '), ' ');
        assert_eq!(full2half('!'), '!');

        // 3. 全角文字を半角に変換
        assert_eq!(full2half('！'), '!'); // 全角感嘆符 → 半角感嘆符
        assert_eq!(full2half('Ａ'), 'A'); // 全角A → 半角A
        assert_eq!(full2half('ａ'), 'a'); // 全角a → 半角a
        assert_eq!(full2half('１'), '1'); // 全角1 → 半角1
        assert_eq!(full2half('＃'), '#'); // 全角# → 半角#

        // 4. 様々なスペース文字を半角スペースに変換
        assert_eq!(full2half('\u{2002}'), ' '); // en space
        assert_eq!(full2half('\u{2003}'), ' '); // em space
        assert_eq!(full2half('\u{2004}'), ' '); // three-per-em space
        assert_eq!(full2half('\u{3000}'), ' '); // ideographic space
        assert_eq!(full2half('\u{FEFF}'), ' '); // zero width no-break space

        // 5. その他の文字はそのまま
        assert_eq!(full2half('あ'), 'あ'); // ひらがな
        assert_eq!(full2half('漢'), '漢'); // 漢字
        assert_eq!(full2half('😀'), '😀'); // 絵文字
    }

    #[test]
    fn test_char_from_u32() {
        // 有効なUnicodeコードポイント
        assert_eq!(char_from_u32(0x0041, '?'), 'A'); // 'A'のコードポイント
        assert_eq!(char_from_u32(0x3042, '?'), 'あ'); // 'あ'のコードポイント

        // 無効なUnicodeコードポイントはデフォルト値を返す
        assert_eq!(char_from_u32(0x110000, '?'), '?'); // 無効なコードポイント
    }

    #[test]
    fn test_string_full2half_comprehensive() {
        // 複合的な変換テスト
        let input = "ＡＢＣ　１２３！＃＄あいう漢字DEFGHI";
        let expected = "ABC 123!#$あいう漢字DEFGHI";
        assert_eq!(string_full2half(input), expected);

        // 空文字列
        assert_eq!(string_full2half(""), "");

        // 変換対象がない文字列
        let no_change = "ABC123!#$あいう漢字";
        assert_eq!(string_full2half(no_change), no_change);
    }
}
