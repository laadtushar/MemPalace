use chrono::Utc;
use uuid::Uuid;

use crate::domain::models::document::Chunk;

/// Configuration for the chunking strategy.
pub struct ChunkerConfig {
    pub max_tokens: usize,
    pub overlap_tokens: usize,
}

impl Default for ChunkerConfig {
    fn default() -> Self {
        Self {
            max_tokens: 512,
            overlap_tokens: 50,
        }
    }
}

/// Split document text into overlapping chunks, respecting paragraph and sentence boundaries.
/// Uses owned strings internally to avoid lifetime issues with cross-paragraph chunks.
pub fn chunk_text(document_id: &str, text: &str, config: &ChunkerConfig) -> Vec<Chunk> {
    if text.trim().is_empty() {
        return Vec::new();
    }

    // Flatten text into owned word tokens
    let paragraphs = split_into_paragraphs(text);
    let mut chunks = Vec::new();
    let mut buffer: Vec<String> = Vec::new();
    let mut position = 0;

    for paragraph in &paragraphs {
        let para_words: Vec<String> = paragraph.split_whitespace().map(String::from).collect();
        let para_len = para_words.len();

        if para_len == 0 {
            continue;
        }

        // If a single paragraph exceeds max, split by sentences then force-split
        if para_len > config.max_tokens {
            // Flush buffer first
            if !buffer.is_empty() {
                chunks.push(make_chunk(document_id, &buffer, position));
                position += 1;
                buffer = take_overlap(&buffer, config.overlap_tokens);
            }

            let sentences = split_into_sentences(paragraph);
            for sentence in &sentences {
                let words: Vec<String> = sentence.split_whitespace().map(String::from).collect();
                // Force-split words that still exceed max (no sentence boundaries found)
                for word in words {
                    if buffer.len() >= config.max_tokens {
                        chunks.push(make_chunk(document_id, &buffer, position));
                        position += 1;
                        buffer = take_overlap(&buffer, config.overlap_tokens);
                    }
                    buffer.push(word);
                }
            }
        } else if buffer.len() + para_len > config.max_tokens {
            // Would overflow — flush
            if !buffer.is_empty() {
                chunks.push(make_chunk(document_id, &buffer, position));
                position += 1;
                buffer = take_overlap(&buffer, config.overlap_tokens);
            }
            buffer.extend(para_words);
        } else {
            buffer.extend(para_words);
        }
    }

    // Flush remaining
    if !buffer.is_empty() {
        chunks.push(make_chunk(document_id, &buffer, position));
    }

    chunks
}

fn take_overlap(buffer: &[String], overlap: usize) -> Vec<String> {
    let start = buffer.len().saturating_sub(overlap);
    buffer[start..].to_vec()
}

fn make_chunk(document_id: &str, words: &[String], position: usize) -> Chunk {
    let text = words.join(" ");
    Chunk {
        id: Uuid::new_v4().to_string(),
        document_id: document_id.to_string(),
        text,
        token_count: words.len(),
        position,
        created_at: Utc::now(),
    }
}

/// Split text into paragraphs (double newline separated).
fn split_into_paragraphs(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(|p| p.trim().to_string())
        .filter(|p| !p.is_empty())
        .collect()
}

/// Split a paragraph into sentences (basic heuristic).
fn split_into_sentences(text: &str) -> Vec<String> {
    let mut sentences = Vec::new();
    let mut current = String::new();

    for ch in text.chars() {
        current.push(ch);
        if (ch == '.' || ch == '!' || ch == '?') && current.len() > 10 {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                sentences.push(trimmed);
            }
            current = String::new();
        }
    }
    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        sentences.push(trimmed);
    }
    sentences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_chunking() {
        let text = "Word ".repeat(100);
        let config = ChunkerConfig {
            max_tokens: 50,
            overlap_tokens: 10,
        };
        let chunks = chunk_text("doc1", text.trim(), &config);
        assert!(chunks.len() >= 2);
        assert!(chunks[0].token_count <= 50);
        assert_eq!(chunks[0].position, 0);
        assert_eq!(chunks[1].position, 1);
    }

    #[test]
    fn test_short_text_single_chunk() {
        let text = "This is a short note.";
        let config = ChunkerConfig::default();
        let chunks = chunk_text("doc1", text, &config);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, text);
    }

    #[test]
    fn test_empty_text() {
        let chunks = chunk_text("doc1", "", &ChunkerConfig::default());
        assert!(chunks.is_empty());
    }

    #[test]
    fn test_paragraph_boundary_respected() {
        let para1 = "Word ".repeat(40);
        let para2 = "Token ".repeat(40);
        let text = format!("{}\n\n{}", para1.trim(), para2.trim());

        let config = ChunkerConfig {
            max_tokens: 50,
            overlap_tokens: 5,
        };
        let chunks = chunk_text("doc1", &text, &config);
        assert!(chunks.len() >= 2);
        assert!(chunks[0].text.contains("Word"));
    }

    #[test]
    fn test_chunks_have_unique_ids() {
        let text = "Word ".repeat(200);
        let config = ChunkerConfig {
            max_tokens: 50,
            overlap_tokens: 10,
        };
        let chunks = chunk_text("doc1", text.trim(), &config);
        let ids: std::collections::HashSet<_> = chunks.iter().map(|c| &c.id).collect();
        assert_eq!(ids.len(), chunks.len());
    }

    #[test]
    fn test_overlap_present() {
        let words: Vec<String> = (0..100).map(|i| format!("w{}", i)).collect();
        let text = words.join(" ");
        let config = ChunkerConfig {
            max_tokens: 30,
            overlap_tokens: 5,
        };
        let chunks = chunk_text("doc1", &text, &config);
        assert!(chunks.len() >= 3);

        // Check that the end of chunk 0 overlaps with the start of chunk 1
        let c0_words: Vec<&str> = chunks[0].text.split_whitespace().collect();
        let c1_words: Vec<&str> = chunks[1].text.split_whitespace().collect();
        let c0_tail: Vec<&&str> = c0_words.iter().rev().take(5).collect();
        let c1_head: Vec<&&str> = c1_words.iter().take(5).collect();
        // At least some overlap should exist
        let overlap_count = c0_tail
            .iter()
            .filter(|w| c1_head.contains(w))
            .count();
        assert!(overlap_count > 0, "Expected overlap between chunks");
    }
}
