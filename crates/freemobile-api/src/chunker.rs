use crate::constants::{
    MAX_MESSAGE_LENGTH, PREFIX_RESERVE_LENGTH, word_boundary::MIN_BOUNDARY_RATIO,
};
use unicode_segmentation::UnicodeSegmentation;

/// Message chunker for handling FreeMobile's length limits
pub struct MessageChunker;

impl MessageChunker {
    /// Splits a message into chunks that fit FreeMobile's 999 character limit
    /// Uses Unicode grapheme-aware splitting to handle complex characters correctly
    /// Reserves space for chunk prefixes like "[1/2] " when multiple chunks are needed
    pub fn chunk(message: &str) -> Vec<String> {
        // Early return for empty or whitespace-only messages
        if message.trim().is_empty() {
            return vec![];
        }

        // For single chunk, use full length limit
        if message.graphemes(true).count() <= MAX_MESSAGE_LENGTH {
            return vec![message.to_string()];
        }

        // For multiple chunks, reserve space for prefixes like "[1/2] "
        let effective_chunk_limit = MAX_MESSAGE_LENGTH - PREFIX_RESERVE_LENGTH;
        let mut chunks = Vec::new();
        let mut current_pos = 0;

        while current_pos < message.len() {
            let remaining = &message[current_pos..];

            // If remaining text is short enough, take it all
            if remaining.graphemes(true).count() <= effective_chunk_limit {
                chunks.push(remaining.trim().to_string());
                break;
            }

            // Build the chunk character by character, tracking the last good word boundary
            let mut chunk_text = String::new();
            let mut last_word_boundary_pos = 0;
            let mut byte_pos = 0;

            for (chars_count, grapheme) in remaining.graphemes(true).enumerate() {
                // Check if adding this grapheme would exceed the limit
                if chars_count >= effective_chunk_limit {
                    break;
                }

                // Add the grapheme
                chunk_text.push_str(grapheme);
                byte_pos += grapheme.len();

                // Update word boundary position if this is whitespace
                if grapheme.chars().any(|c| c.is_whitespace()) {
                    last_word_boundary_pos = byte_pos;
                }
            }

            // If we found a word boundary and it's not too close to the beginning, use it
            let split_pos = if last_word_boundary_pos > byte_pos / MIN_BOUNDARY_RATIO {
                last_word_boundary_pos
            } else {
                byte_pos
            };

            if split_pos > 0 {
                let chunk_text = &remaining[..split_pos];
                chunks.push(chunk_text.trim().to_string());
                current_pos += split_pos;

                // Skip any whitespace at the start of the next chunk
                while current_pos < message.len() {
                    let ch = message[current_pos..].chars().next().unwrap();
                    if ch.is_whitespace() {
                        current_pos += ch.len_utf8();
                    } else {
                        break;
                    }
                }
            } else {
                // Safety fallback: take at least one character
                let next_char = remaining.chars().next().unwrap();
                chunks.push(next_char.to_string());
                current_pos += next_char.len_utf8();
            }
        }

        chunks
    }

    /// Formats chunks with index prefixes for multi-part messages
    pub fn format_chunks(chunks: &[String]) -> Vec<String> {
        if chunks.len() <= 1 {
            return chunks.to_vec();
        }

        chunks
            .iter()
            .enumerate()
            .map(|(index, chunk)| format!("[{}/{}] {}", index + 1, chunks.len(), chunk))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_message_no_chunking() {
        let message = "Hello world!";
        let chunks = MessageChunker::chunk(message);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], message);
    }

    #[test]
    fn test_long_message_chunking() {
        let message = "a".repeat(1500);
        let chunks = MessageChunker::chunk(&message);
        assert!(chunks.len() > 1);

        // Test that raw chunks fit within effective limit
        let effective_limit = MAX_MESSAGE_LENGTH - PREFIX_RESERVE_LENGTH;
        assert!(
            chunks
                .iter()
                .all(|chunk| chunk.graphemes(true).count() <= effective_limit)
        );

        // Test that formatted chunks fit within API limit
        let formatted = MessageChunker::format_chunks(&chunks);
        assert!(
            formatted
                .iter()
                .all(|chunk| chunk.graphemes(true).count() <= MAX_MESSAGE_LENGTH)
        );
    }

    #[test]
    fn test_unicode_awareness() {
        let message = "🌟".repeat(500) + &"⭐".repeat(600);
        let chunks = MessageChunker::chunk(&message);
        assert!(chunks.len() > 1);

        // Test formatted chunks respect API limit
        let formatted = MessageChunker::format_chunks(&chunks);
        assert!(
            formatted
                .iter()
                .all(|chunk| chunk.graphemes(true).count() <= MAX_MESSAGE_LENGTH)
        );
    }

    #[test]
    fn test_format_single_chunk() {
        let chunks = vec!["Single message".to_string()];
        let formatted = MessageChunker::format_chunks(&chunks);
        assert_eq!(formatted.len(), 1);
        assert_eq!(formatted[0], "Single message");
    }

    #[test]
    fn test_format_multiple_chunks() {
        let chunks = vec!["First chunk".to_string(), "Second chunk".to_string()];
        let formatted = MessageChunker::format_chunks(&chunks);
        assert_eq!(formatted.len(), 2);
        assert_eq!(formatted[0], "[1/2] First chunk");
        assert_eq!(formatted[1], "[2/2] Second chunk");
    }

    #[test]
    fn test_empty_chunk_handling() {
        let message = "   \n\n   ";
        let chunks = MessageChunker::chunk(message);
        assert_eq!(chunks.len(), 0);
    }

    #[test]
    fn test_whitespace_trimming() {
        let message = "a".repeat(500) + "   \n\n   " + &"b".repeat(600);
        let chunks = MessageChunker::chunk(&message);
        assert!(
            chunks
                .iter()
                .all(|chunk| !chunk.starts_with(' ') && !chunk.ends_with(' '))
        );
    }

    #[test]
    fn test_real_world_long_message() {
        // Test with ~1328 characters (similar to long-example.txt)
        let message = "lorem ipsum ".repeat(100); // ~1200 chars
        let chunks = MessageChunker::chunk(&message);

        // Should create multiple chunks
        assert!(chunks.len() > 1);

        // All formatted chunks must fit API limit
        let formatted = MessageChunker::format_chunks(&chunks);
        assert!(
            formatted
                .iter()
                .all(|chunk| chunk.graphemes(true).count() <= MAX_MESSAGE_LENGTH),
            "Formatted chunk exceeds API limit: {:?}",
            formatted.iter().map(|c| c.len()).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_word_boundary_respect() {
        // Test that words are not broken in the middle
        let words = vec![
            "hello",
            "world",
            "testing",
            "message",
            "chunking",
            "functionality",
        ];
        let repeated_sentence = format!("{} ", words.join(" "));
        let message = repeated_sentence.repeat(20); // Create a long message with proper spaces
        let chunks = MessageChunker::chunk(&message);

        assert!(chunks.len() > 1);

        // Check that chunks prefer word boundaries
        for chunk in &chunks {
            // Skip empty chunks
            if chunk.trim().is_empty() {
                continue;
            }

            // Each chunk should contain complete words
            let chunk_words: Vec<&str> = chunk.split_whitespace().collect();
            assert!(
                !chunk_words.is_empty(),
                "Chunk should not be empty: '{}'",
                chunk
            );

            // Most words should be from our original word list (allowing for some edge cases)
            let recognized_words = chunk_words
                .iter()
                .filter(|word| words.iter().any(|original| *word == original))
                .count();

            // At least MIN_WORD_RECOGNITION_RATIO of words should be recognized (allowing for some split words in edge cases)
            assert!(
                recognized_words as f64 / chunk_words.len() as f64
                    >= crate::constants::word_boundary::MIN_WORD_RECOGNITION_RATIO,
                "Too many unrecognized words in chunk '{}'. Words: {:?}",
                chunk,
                chunk_words
            );
        }
    }

    #[test]
    fn test_very_long_single_word() {
        // Test behavior with a single word that's longer than the chunk limit
        let very_long_word = "a".repeat(1000);
        let chunks = MessageChunker::chunk(&very_long_word);

        assert!(chunks.len() > 1);

        // All formatted chunks should fit the API limit
        let formatted = MessageChunker::format_chunks(&chunks);
        assert!(
            formatted
                .iter()
                .all(|chunk| chunk.graphemes(true).count() <= MAX_MESSAGE_LENGTH)
        );
    }

    #[test]
    fn test_long_example_file_content() {
        // Test with the exact content of long-example.txt (1328 chars)
        let content = "lorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum.\n\nlorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum.\n\nlorem ipsum dolor sit amet consectetur adipiscing elit sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident sunt in culpa qui officia deserunt mollit anim id est laborum.\n";

        let chunks = MessageChunker::chunk(content);
        let formatted = MessageChunker::format_chunks(&chunks);

        // Should create exactly 2 chunks for 1328 characters (not 3)
        assert_eq!(
            chunks.len(),
            2,
            "Expected exactly 2 chunks for 1328 chars, got {}: {:?}",
            chunks.len(),
            chunks.iter().map(|c| c.len()).collect::<Vec<_>>()
        );

        // All formatted chunks should fit the API limit
        assert!(
            formatted
                .iter()
                .all(|chunk| chunk.graphemes(true).count() <= MAX_MESSAGE_LENGTH),
            "Formatted chunks exceed API limit: {:?}",
            formatted.iter().map(|c| c.len()).collect::<Vec<_>>()
        );

        // Check that words are not broken (allow some common short words at boundaries)
        for (i, chunk) in chunks.iter().enumerate() {
            let words: Vec<&str> = chunk.split_whitespace().collect();
            if let Some(last_word) = words.last() {
                // Allow short connecting words like "et", "de", "la", etc. at chunk boundaries
                let is_acceptable_boundary_word = last_word.len()
                    <= crate::constants::word_boundary::MIN_ACCEPTABLE_WORD_LENGTH
                    && matches!(
                        *last_word,
                        "et" | "de" | "la" | "le" | "du" | "un" | "en" | "au" | "ou" | "si"
                    );

                assert!(
                    last_word.len() > crate::constants::word_boundary::MIN_ACCEPTABLE_WORD_LENGTH
                        || is_acceptable_boundary_word
                        || last_word.chars().all(|c| c.is_ascii_punctuation()),
                    "Chunk {} ends with unexpected short word: '{}' (full chunk ending: '{}')",
                    i + 1,
                    last_word,
                    chunk
                        .chars()
                        .rev()
                        .take(30)
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect::<String>()
                );
            }
        }
    }
}
