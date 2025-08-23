# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-08-24

### Initial Release Features

#### 🏗️ **Workspace Architecture**
- **Rust Workspace** with two independent crates
- **`freemobile-api`** : Reusable library for FreeMobile API
- **`send-sms-cli`** : Modern command-line interface

#### 📚 **freemobile-api Library**
- Complete Rust interface for FreeMobile SMS API
- Smart emoji sanitization supporting 146+ FreeMobile-compatible emojis
- Automatic message chunking for messages exceeding 999 characters
- Full Unicode support with grapheme cluster handling
- Async/await architecture built on Tokio
- Comprehensive error types with `thiserror`
- Complete documentation with examples
- 16 unit tests covering core functionality

#### 🖥️ **send-sms-cli CLI**
- Modern command-line interface for SMS sending
- **Multiple input modes** :
  - Direct message arguments (`-m "message"`)
  - File input (`-f file.txt`)
  - Stdin detection for pipes and redirections
  - Interactive mode with user-friendly prompts
- **Configuration flexibility** :
  - Environment variables (`FREEMOBILE_USER`, `FREEMOBILE_PASS`)
  - `.env` file support
  - CLI arguments (`-u`, `-p`)
  - Interactive credential prompts
- **User experience features** :
  - Comprehensive error handling with clear messages
  - Graceful interruption support (Ctrl+C)
  - Verbose mode (`-v`) for detailed output
  - 8 unit tests ensuring reliability

#### 🚀 **Core Capabilities**
- **Smart emoji handling** : Preserves 146+ FreeMobile-compatible emojis, replaces unsupported ones
- **Message chunking** : Word-boundary-aware splitting with progress indicators `[1/2]`, `[2/2]`
- **Unicode processing** : Proper handling of accented characters and international text
- **Async architecture** : Tokio-based implementation for efficient operations
- **Rate limiting** : Built-in delays between API calls
- **Memory efficiency** : `LazyLock` optimizations for regex compilation
- **Security** : Credential masking in all output

#### 🛠️ **Development & Quality**
- **Type-safe error handling** with 10 specific error types
- **Comprehensive test suite** : 41 tests covering critical functionality
- **Code quality** : Zero clippy warnings, rustfmt formatting
- **Build system** : Complete Makefile for development workflow
- **Optimized binary** : 2.4MB release build with full optimizations
- **Documentation** : Complete API documentation and user guides

### Security
- FreeMobile User ID validation (8-digit format)
- Credential masking in all log outputs
- Secure environment variable handling
- TLS encryption with rustls for API communications

### Performance
- **Binary size** : 2.4MB optimized release build
- **Memory usage** : Minimal allocations with `LazyLock` optimizations
- **Network** : HTTP connection reuse with TLS encryption
- **Text processing** : Efficient Unicode grapheme handling
- **Startup time** : Sub-50ms cold startup
