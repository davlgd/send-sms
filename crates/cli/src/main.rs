use freemobile_api::{FreeMobileClient, FreeMobileError, MessageSanitizer};
use send_sms::{Config, InputHandler};
use std::process;
use tokio::signal;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("❌ Error: {}", e);
        process::exit(1);
    }
}

async fn run() -> Result<(), FreeMobileError> {
    // Set up signal handling for graceful shutdown
    tokio::spawn(async {
        signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
        println!("\n\n🛑 Interrupted by user");
        process::exit(130); // Standard exit code for SIGINT
    });

    // Parse configuration
    let config = Config::from_args()?;

    if config.verbose {
        println!("🚀 Starting send-sms v{}", env!("CARGO_PKG_VERSION"));
        println!("📱 User ID: {}", mask_user_id(&config.credentials.user));
    }

    // Initialize FreeMobile client
    let client = FreeMobileClient::new(config.credentials.clone())?;

    // Get message from various sources
    let message = get_message(&config).await?;

    // Validate original message
    InputHandler::validate_message(&message)?;

    // Sanitize for sending
    let sanitized_message = MessageSanitizer::sanitize(&message);

    // Preview the message (what will actually be sent)
    let debug_mode = std::env::var("DEBUG").is_ok() || std::env::var("RUST_LOG").is_ok();

    // In debug mode, show original message if it was modified
    if debug_mode && sanitized_message != message {
        use unicode_segmentation::UnicodeSegmentation;
        let truncated: String = message.graphemes(true).take(50).collect();
        println!("🐛 DEBUG - Original message: {}...", truncated);
        println!("🐛 DEBUG - Sanitized message (what will be sent):");
    }

    // Always show the sanitized message (what will actually be sent)
    InputHandler::preview_message(&sanitized_message, config.verbose);

    // Send the sanitized message
    if config.verbose {
        println!("📤 Sending SMS...");
    }

    // Send the already-sanitized message
    client.send_sanitized(&sanitized_message).await?;

    if config.verbose {
        println!("✅ SMS sent successfully!");
    } else {
        println!("✅ SMS sent");
    }

    Ok(())
}

async fn get_message(config: &Config) -> Result<String, FreeMobileError> {
    // Priority 1: Direct message via CLI argument
    if let Some(ref message) = config.message {
        return Ok(message.clone());
    }

    // Priority 2: File input
    if let Some(ref file_path) = config.file_path {
        if config.verbose {
            println!("📁 Reading message from file: {}", file_path.display());
        }
        return InputHandler::get_message_from_file(file_path).await;
    }

    // Priority 3: Auto-detect stdin input (pipe or redirect)
    if InputHandler::has_stdin_input() {
        if config.verbose {
            println!("📥 Detected stdin input...");
        }
        return InputHandler::get_message_from_stdin().await;
    }

    // Priority 4: Interactive mode (default fallback)
    if config.verbose {
        println!("💬 No input detected, using interactive mode...");
    }
    InputHandler::get_message_interactive().await
}

fn mask_user_id(user_id: &str) -> String {
    if user_id.len() >= 4 {
        format!("{}****", &user_id[..4])
    } else {
        "****".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_user_id() {
        assert_eq!(mask_user_id("12345678"), "1234****");
        assert_eq!(mask_user_id("123"), "****");
        assert_eq!(mask_user_id(""), "****");
    }

    #[test]
    fn test_debug_mode_detection() {
        use std::env;

        // Save original state
        let original_debug = env::var("DEBUG").ok();
        let original_rust_log = env::var("RUST_LOG").ok();

        unsafe {
            // Test no debug vars
            env::remove_var("DEBUG");
            env::remove_var("RUST_LOG");
            let debug_mode = env::var("DEBUG").is_ok() || env::var("RUST_LOG").is_ok();
            assert!(!debug_mode);

            // Test DEBUG var
            env::set_var("DEBUG", "1");
            let debug_mode = env::var("DEBUG").is_ok() || env::var("RUST_LOG").is_ok();
            assert!(debug_mode);

            // Test RUST_LOG var
            env::remove_var("DEBUG");
            env::set_var("RUST_LOG", "debug");
            let debug_mode = env::var("DEBUG").is_ok() || env::var("RUST_LOG").is_ok();
            assert!(debug_mode);

            // Restore original state
            match original_debug {
                Some(val) => env::set_var("DEBUG", val),
                None => env::remove_var("DEBUG"),
            }
            match original_rust_log {
                Some(val) => env::set_var("RUST_LOG", val),
                None => env::remove_var("RUST_LOG"),
            }
        }
    }
}
