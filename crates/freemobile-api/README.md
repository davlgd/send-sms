# FreeMobile API - Rust Library

[![Crates.io](https://img.shields.io/crates/v/freemobile-api)](https://crates.io/crates/freemobile-api)
[![Documentation](https://docs.rs/freemobile-api/badge.svg)](https://docs.rs/freemobile-api)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

A modern, efficient, and safe Rust library for FreeMobile SMS API integration. Built with async/await support, comprehensive emoji handling, and production-ready reliability.

## Features

- 🚀 **Async/Await Support** - Built on Tokio for high-performance concurrent operations
- 📱 **FreeMobile API Integration** - Complete SMS sending functionality with proper error handling
- 🌍 **Smart Emoji Handling** - Supports 146+ FreeMobile-compatible emojis with intelligent sanitization
- 📝 **Automatic Message Chunking** - Handles long messages (>999 chars) with progress indicators
- 🦀 **Memory Efficient** - Minimal allocations with LazyLock regex compilation
- 🛡️ **Type-Safe Error Handling** - Comprehensive error types with detailed context
- 🌐 **Unicode Aware** - Full grapheme cluster support for international text
- 📚 **Well Documented** - Complete API documentation with examples

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
freemobile-api = "0.1.0"
tokio = { version = "1.0", features = ["rt-multi-thread", "macros"] }
```

### Basic Usage

```rust
use freemobile_api::{FreeMobileClient, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create credentials
    let credentials = Credentials::new(
        "12345678".to_string(),        // Your FreeMobile user ID (8 digits)
        "your-api-key".to_string()     // Your API key from FreeMobile
    );
    
    // Initialize client
    let client = FreeMobileClient::new(credentials)?;
    
    // Send a message
    client.send("Hello from Rust! ✅").await?;
    
    println!("Message sent successfully!");
    Ok(())
}
```

### Advanced Usage

```rust
use freemobile_api::{FreeMobileClient, Credentials, FreeMobileError};

#[tokio::main]
async fn main() {
    let credentials = Credentials::new(
        std::env::var("FREEMOBILE_USER").expect("FREEMOBILE_USER not set"),
        std::env::var("FREEMOBILE_PASS").expect("FREEMOBILE_PASS not set")
    );
    
    let client = FreeMobileClient::new(credentials).unwrap();
    
    // Send with comprehensive error handling
    match client.send("🚀 Deployment complete! All tests passed ✅").await {
        Ok(()) => println!("✅ SMS sent successfully"),
        Err(FreeMobileError::InvalidCredentials) => {
            eprintln!("❌ Invalid credentials - check your user ID and API key");
        }
        Err(FreeMobileError::TooManyRequests) => {
            eprintln!("⏳ Rate limit exceeded - please wait before sending again");
        }
        Err(FreeMobileError::EmptyMessage) => {
            eprintln!("📝 Message is empty");
        }
        Err(e) => {
            eprintln!("💥 Unexpected error: {}", e);
        }
    }
    
    // Preview sanitization without sending
    let original = "Hello 😀 world 🚀 with ✅ mixed 📱 emojis!";
    let sanitized = client.sanitize_message(original);
    println!("Original:  {}", original);
    println!("Sanitized: {}", sanitized); // "Hello [] world [] with ✅ mixed [] emojis!"
}
```

## Message Processing

### Emoji Sanitization

The library automatically handles emoji compatibility:

- **Supported emojis** (146+ emojis): ✅ ❌ ⚡ ⭐ ❤️ ☀️ ⚠️ etc. → **Preserved**
- **Unsupported emojis**: 😀 🚀 🎉 💻 📱 etc. → **Replaced with []**
- **Accented characters**: café, résumé, naïf → **Always preserved**

```rust
let client = FreeMobileClient::new(credentials)?;

// Emojis are automatically sanitized
client.send("Status: ✅ OK, Performance: ⚡ Fast, Issues: 😓 None").await?;
// Actually sends: "Status: ✅ OK, Performance: ⚡ Fast, Issues: [] None"
```

### Message Chunking

Long messages are automatically split into chunks:

```rust
let long_message = "A".repeat(2000); // 2000 characters
client.send(&long_message).await?;

// Automatically sends as:
// "[1/2] AAAA..." (first 999 chars)  
// "[2/2] AAAA..." (remaining chars)
```

## Error Handling

The library provides comprehensive, typed error handling:

```rust
use freemobile_api::FreeMobileError;

match client.send("message").await {
    Ok(()) => {
        // Message sent successfully
    }
    Err(FreeMobileError::InvalidCredentials) => {
        // HTTP 400 - Check user ID and API key
    }
    Err(FreeMobileError::TooManyRequests) => {
        // HTTP 402 - Rate limit exceeded, wait and retry
    }
    Err(FreeMobileError::AccessDenied) => {
        // HTTP 403 - Check FreeMobile subscription status
    }
    Err(FreeMobileError::ServerError) => {
        // HTTP 500 - FreeMobile server error
    }
    Err(FreeMobileError::HttpError(e)) => {
        // Network or HTTP client error
        eprintln!("Network error: {}", e);
    }
    Err(FreeMobileError::EmptyMessage) => {
        // Message was empty after trimming
    }
    Err(e) => {
        // Other errors
        eprintln!("Error: {}", e);
    }
}
```

## Configuration

### Environment Variables

```bash
export FREEMOBILE_USER="12345678"     # Your 8-digit user ID
export FREEMOBILE_PASS="your-api-key" # Your API key
```

### Getting FreeMobile Credentials

1. Log into your FreeMobile account
2. Go to "Gérer mon compte" → "Mes options"  
3. Enable "Notification par SMS"
4. Note your User ID (8 digits) and API key

## Performance

- **Memory efficient**: LazyLock regex compilation, minimal allocations
- **Network optimized**: Async HTTP with connection reuse and proper timeouts
- **Unicode aware**: Grapheme-cluster-based text processing for accurate character counting
- **Rate limiting**: Built-in delays between message chunks (500ms)

## Thread Safety

`FreeMobileClient` is `Send` and `Sync`, making it safe to use across async tasks:

```rust
use std::sync::Arc;

let client = Arc::new(FreeMobileClient::new(credentials)?);

// Use in multiple async tasks
let client_clone = client.clone();
tokio::spawn(async move {
    client_clone.send("Message from task 1").await.unwrap();
});

let client_clone = client.clone();  
tokio::spawn(async move {
    client_clone.send("Message from task 2").await.unwrap();
});
```

## Supported Emojis

The library supports 146+ emojis that are compatible with FreeMobile's SMS system:

**Symbols**: ⚡ ✅ ❌ ⚠️ ⭐ ❤️ ☀️ ☁️ ❄️ ⛄ ⛅  
**Numbers**: 0️⃣ 1️⃣ 2️⃣ 3️⃣ 4️⃣ 5️⃣ 6️⃣ 7️⃣ 8️⃣ 9️⃣  
**Arrows**: ➡️ ⬅️ ⬆️ ⬇️ ↗️ ↘️ ↙️ ↖️  
**Shapes**: ⬛ ⬜ ◼️ ◻️ ◾ ◽ ⚪ ⚫  

[View complete emoji list](src/supported_emojis.rs)

## Configuration

All API parameters are configurable through constants in `src/constants.rs`:

- **Message limits**: MAX_MESSAGE_LENGTH (999 chars), PREFIX_RESERVE_LENGTH (8 chars)
- **Network settings**: REQUEST_TIMEOUT_SECS (30s), CHUNK_DELAY_MS (500ms)  
- **Word processing**: MIN_ACCEPTABLE_WORD_LENGTH, MIN_BOUNDARY_RATIO
- **API endpoints**: URL and user agent string

## Requirements

- Rust 2024 edition (MSRV: 1.88+)
- Tokio runtime for async operations
- Active FreeMobile subscription with API access

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../LICENSE) for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.
