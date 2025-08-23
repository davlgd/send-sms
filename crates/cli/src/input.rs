use crate::constants::{MAX_MESSAGE_LENGTH, MESSAGE_PREVIEW_LENGTH};
use freemobile_api::FreeMobileError;
use inquire::Text;
use std::fs;
use std::io::{self, Read};
use std::path::Path;

pub struct InputHandler;

impl InputHandler {
    pub async fn get_message_from_file<P: AsRef<Path>>(path: P) -> Result<String, FreeMobileError> {
        let content = fs::read_to_string(path).map_err(FreeMobileError::IoError)?;

        if content.trim().is_empty() {
            return Err(FreeMobileError::EmptyMessage);
        }

        Ok(content.trim().to_string())
    }

    pub async fn get_message_from_stdin() -> Result<String, FreeMobileError> {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .map_err(FreeMobileError::IoError)?;

        if buffer.trim().is_empty() {
            return Err(FreeMobileError::EmptyMessage);
        }

        Ok(buffer.trim().to_string())
    }

    pub async fn get_message_interactive() -> Result<String, FreeMobileError> {
        println!("Enter your message (press Enter to send, Ctrl+C to cancel):");

        let message = Text::new("")
            .with_placeholder("Type your message here...")
            .prompt()
            .map_err(|e| {
                FreeMobileError::ConfigError(format!("Interactive input failed: {}", e))
            })?;

        if message.trim().is_empty() {
            return Err(FreeMobileError::EmptyMessage);
        }

        Ok(message.trim().to_string())
    }

    pub fn validate_message(message: &str) -> Result<(), FreeMobileError> {
        if message.trim().is_empty() {
            return Err(FreeMobileError::EmptyMessage);
        }

        // Check for potential issues
        if message.len() > MAX_MESSAGE_LENGTH {
            return Err(FreeMobileError::InvalidMessage(format!(
                "Message too long (maximum {} characters)",
                MAX_MESSAGE_LENGTH
            )));
        }

        Ok(())
    }

    pub fn preview_message(message: &str, verbose: bool) {
        if !verbose {
            return;
        }

        println!("📄 Message preview:");
        println!("Length: {} characters", message.len());

        if message.len() > MESSAGE_PREVIEW_LENGTH {
            use unicode_segmentation::UnicodeSegmentation;
            let truncated: String = message
                .graphemes(true)
                .take(MESSAGE_PREVIEW_LENGTH)
                .collect();
            println!(
                "Content (first {} graphemes): {}",
                MESSAGE_PREVIEW_LENGTH, truncated
            );
            println!("... (truncated for preview)");
        } else {
            println!("Content: {}", message);
        }
        println!();
    }

    pub fn has_stdin_input() -> bool {
        use is_terminal::IsTerminal;
        !io::stdin().is_terminal()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_read_from_file() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "Test message from file").unwrap();

        let message = InputHandler::get_message_from_file(temp_file.path())
            .await
            .unwrap();
        assert_eq!(message, "Test message from file");
    }

    #[tokio::test]
    async fn test_read_from_empty_file() {
        let temp_file = NamedTempFile::new().unwrap();

        let result = InputHandler::get_message_from_file(temp_file.path()).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FreeMobileError::EmptyMessage));
    }

    #[test]
    fn test_validate_message() {
        assert!(InputHandler::validate_message("Valid message").is_ok());
        assert!(InputHandler::validate_message("").is_err());
        assert!(InputHandler::validate_message("   ").is_err());

        // Test maximum length boundary
        let valid_message = "a".repeat(MAX_MESSAGE_LENGTH);
        assert!(InputHandler::validate_message(&valid_message).is_ok());

        let too_long_message = "a".repeat(MAX_MESSAGE_LENGTH + 1);
        assert!(InputHandler::validate_message(&too_long_message).is_err());
    }

    #[test]
    fn test_message_preview() {
        // Test with verbose = false (should not print anything)
        InputHandler::preview_message("Test message", false);

        // Test with verbose = true (would print to stdout in real usage)
        InputHandler::preview_message("Test message", true);

        let long_message = "a".repeat(150);
        InputHandler::preview_message(&long_message, true);
    }
}
