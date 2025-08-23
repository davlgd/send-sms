//! FreeMobile API - Rust library for FreeMobile SMS integration
//!
//! This crate provides a simple, efficient, and safe Rust interface for sending SMS messages
//! via the FreeMobile API. It includes emoji sanitization, automatic message chunking with
//! word-boundary awareness, and configurable parameters.
//!
//! ## Features
//!
//! - **FreeMobile SMS API integration** with proper error handling
//! - **Smart emoji sanitization** supporting 146+ FreeMobile-compatible emojis  
//! - **Automatic message chunking** for messages exceeding 999 characters
//! - **Configurable constants** externalized in `constants` module
//! - **Word-boundary-aware splitting** to avoid breaking words mid-sentence
//! - **Unicode-aware processing** with proper grapheme cluster handling
//! - **Async/await support** built on Tokio
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use freemobile_api::{FreeMobileClient, Credentials};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let credentials = Credentials::new(
//!         "12345678".to_string(),
//!         "your-api-key".to_string()
//!     );
//!     
//!     let client = FreeMobileClient::new(credentials)?;
//!     
//!     client.send("Hello from Rust! ✅").await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! All parameters are configurable via the `constants` module:
//!
//! ```rust
//! use freemobile_api::constants::*;
//!
//! // Message limits
//! println!("Max message length: {}", MAX_MESSAGE_LENGTH); // 999
//! println!("Prefix reserve: {}", PREFIX_RESERVE_LENGTH);   // 8
//!
//! // Network timeouts
//! println!("Request timeout: {}s", REQUEST_TIMEOUT_SECS); // 30
//! println!("Chunk delay: {}ms", CHUNK_DELAY_MS);          // 500
//! ```
//!
//! ## Message Processing
//!
//! The library automatically handles:
//!
//! - **Emoji sanitization**: Preserves 146+ supported emojis (✅ ⚡ ❌ ⭐), replaces others with []
//! - **Smart chunking**: Word-boundary-aware splitting with [1/2], [2/2] prefixes
//! - **Unicode processing**: Proper grapheme cluster handling for international text
//!
//! ## Error Handling
//!
//! All API errors are properly typed and provide clear context:
//!
//! ```rust,no_run
//! use freemobile_api::{FreeMobileClient, FreeMobileError, Credentials};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! # let credentials = Credentials::new("12345678".to_string(), "key".to_string());
//! # let client = FreeMobileClient::new(credentials)?;
//! match client.send("message").await {
//!     Ok(()) => println!("Message sent successfully"),
//!     Err(FreeMobileError::InvalidCredentials) => println!("Check your API credentials"),
//!     Err(FreeMobileError::TooManyRequests) => println!("Rate limit exceeded"),
//!     Err(e) => println!("Other error: {}", e),
//! }
//! # Ok(())
//! # }
//! ```

pub mod chunker;
pub mod client;
pub mod constants;
pub mod error;
pub mod sanitizer;
pub mod supported_emojis;

pub use chunker::MessageChunker;
pub use client::{Credentials, FreeMobileClient};
pub use error::FreeMobileError;
pub use sanitizer::MessageSanitizer;
