use crate::supported_emojis::is_supported_emoji;
use regex::Regex;
use std::sync::LazyLock;

/// Static regex for emoji detection, compiled once at startup
static EMOJI_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"[\p{Emoji_Presentation}\p{Extended_Pictographic}][\u{FE0F}\u{20E3}]?")
        .expect("Invalid emoji regex")
});

/// Message sanitizer for FreeMobile API compatibility
pub struct MessageSanitizer;

impl MessageSanitizer {
    /// Sanitizes a message by preserving supported emojis and replacing unsupported ones with []
    pub fn sanitize(message: &str) -> String {
        EMOJI_REGEX
            .replace_all(message, |caps: &regex::Captures| {
                let emoji = &caps[0];
                let normalized = emoji.replace('\u{FE0F}', "");

                if is_supported_emoji(&normalized) || is_supported_emoji(emoji) {
                    emoji.to_string()
                } else {
                    "[]".to_string()
                }
            })
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supported_emojis_preserved() {
        assert_eq!(MessageSanitizer::sanitize("⚡"), "⚡");
        assert_eq!(MessageSanitizer::sanitize("✅"), "✅");
        assert_eq!(MessageSanitizer::sanitize("❌"), "❌");
    }

    #[test]
    fn test_unsupported_emojis_replaced() {
        assert_eq!(MessageSanitizer::sanitize("😀"), "[]");
        assert_eq!(MessageSanitizer::sanitize("🚀"), "[]");
        assert_eq!(MessageSanitizer::sanitize("📱"), "[]");
    }

    #[test]
    fn test_variation_selectors() {
        assert_eq!(MessageSanitizer::sanitize("⚡️"), "⚡️");
        assert_eq!(MessageSanitizer::sanitize("✔️"), "✔️");
    }

    #[test]
    fn test_mixed_content() {
        let input = "Test: ✅ supported 😀 unsupported ⚡ supported";
        let expected = "Test: ✅ supported [] unsupported ⚡ supported";
        assert_eq!(MessageSanitizer::sanitize(input), expected);
    }

    #[test]
    fn test_accents_preserved() {
        let input = "Café résumé naïf";
        assert_eq!(MessageSanitizer::sanitize(input), input);
    }

    #[test]
    fn test_no_emojis() {
        let input = "Simple text message";
        assert_eq!(MessageSanitizer::sanitize(input), input);
    }
}
