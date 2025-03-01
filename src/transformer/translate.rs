
// 言語別の翻訳関数
pub(crate) fn translate(filename: &str, language: &str) -> String {
    match language {
        "ja" => translate_to_japanese(filename),
        "en" => translate_to_english(filename),
        _ => {
            log::warn!("Unsupported language: {}", language);
            filename.to_string()
        }
    }
}

fn translate_to_japanese(filename: &str) -> String {
    // 日本語への翻訳ロジック（例示のみ）
    format!("ja_{}", filename)
}

fn translate_to_english(filename: &str) -> String {
    // 英語への翻訳ロジック
    format!("en_{}", filename)
}

