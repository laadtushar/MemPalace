use unicode_normalization::UnicodeNormalization;

use crate::domain::models::document::Document;

/// Normalize document text: Unicode NFC, whitespace cleanup, encoding fixes.
pub fn normalize_document(doc: &mut Document) {
    doc.raw_text = normalize_text(&doc.raw_text);
}

/// Normalize a batch of documents in place.
pub fn normalize_documents(docs: &mut [Document]) {
    for doc in docs.iter_mut() {
        normalize_document(doc);
    }
}

/// Core text normalization:
/// - Unicode NFC normalization
/// - Replace common encoding artifacts
/// - Normalize whitespace (collapse multiple newlines, trim)
/// - Remove null bytes
fn normalize_text(text: &str) -> String {
    let mut result: String = text.nfc().collect();

    // Remove null bytes
    result = result.replace('\0', "");

    // Replace common encoding artifacts
    result = result
        .replace('\u{FEFF}', "") // BOM
        .replace('\u{200B}', "") // Zero-width space
        .replace('\u{200C}', "") // Zero-width non-joiner
        .replace('\u{200D}', "") // Zero-width joiner
        .replace("\r\n", "\n")   // Windows line endings
        .replace('\r', "\n");    // Old Mac line endings

    // Collapse 3+ consecutive newlines into 2
    while result.contains("\n\n\n") {
        result = result.replace("\n\n\n", "\n\n");
    }

    // Trim leading/trailing whitespace
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_unicode() {
        // NFC normalization: é (e + combining accent) → é (single codepoint)
        let input = "caf\u{0065}\u{0301}"; // e + combining acute
        let result = normalize_text(input);
        assert_eq!(result, "caf\u{00E9}"); // precomposed é
    }

    #[test]
    fn test_collapse_newlines() {
        let input = "Hello\n\n\n\n\nWorld";
        let result = normalize_text(input);
        assert_eq!(result, "Hello\n\nWorld");
    }

    #[test]
    fn test_remove_bom_and_zero_width() {
        let input = "\u{FEFF}Hello\u{200B}World";
        let result = normalize_text(input);
        assert_eq!(result, "HelloWorld");
    }

    #[test]
    fn test_windows_line_endings() {
        let input = "Line 1\r\nLine 2\r\nLine 3";
        let result = normalize_text(input);
        assert_eq!(result, "Line 1\nLine 2\nLine 3");
    }
}
