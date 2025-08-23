# Claude Code Configuration

This file contains configuration and context for Claude Code to work more effectively with this project.

## Project Overview

**send-sms** is a Rust workspace containing two crates:
- `freemobile-api`: Reusable library for FreeMobile SMS API
- `send-sms`: Command-line interface for sending SMS

## Key Commands

### Build & Development
```bash
# Build the project
make build

# Release build
make release

# Run tests  
make test

# Full validation (fmt + clippy + test)
make validate

# Install CLI tool
make install

# Clean build artifacts
make clean

# Show help
make help
```

### Testing
```bash
# Run all tests
cargo test

# Run tests for specific crate
cargo test -p freemobile-api
cargo test -p send-sms

# Run tests with verbose output
cargo test -- --nocapture
```

### Running
```bash
# Run the CLI
cargo run -p send-sms -- --help

# Or after installation
send-sms --help
```

## Project Structure

```
send-sms/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── freemobile-api/     # API library crate
│   │   ├── Cargo.toml
│   │   ├── src/
│   │   │   ├── lib.rs         # Library entry point
│   │   │   ├── constants.rs   # Configurable parameters
│   │   │   ├── client.rs      # HTTP client implementation
│   │   │   ├── error.rs       # Error types
│   │   │   ├── sanitizer.rs   # Emoji sanitization
│   │   │   ├── chunker.rs     # Word-aware message chunking
│   │   │   └── supported_emojis.rs  # Emoji definitions
│   │   └── README.md
│   └── cli/               # CLI crate (send-sms)
│       ├── Cargo.toml
│       ├── src/
│       │   ├── main.rs        # CLI entry point
│       │   ├── lib.rs         # CLI library code
│       │   ├── constants.rs   # CLI-specific limits
│       │   ├── config.rs      # Configuration + interactive prompts
│       │   └── input.rs       # Multi-source input processing
│       └── README.md
```

## Key Technologies

- **Language**: Rust 2024 edition (MSRV: 1.88+)  
- **Async Runtime**: Tokio
- **HTTP Client**: reqwest with rustls-tls
- **CLI**: clap v4
- **Configuration**: dotenv for .env files
- **Error Handling**: thiserror
- **Text Processing**: unicode-segmentation
- **Interactive**: inquire for user prompts

## Configuration

The CLI supports multiple configuration methods:
1. Environment variables: `FREEMOBILE_USER`, `FREEMOBILE_PASS`
2. `.env` files
3. Command-line arguments

## Testing Notes

- Total test coverage: 29 tests (24 unit tests + 5 doctests)
- Tests are split between both crates
- Use `make test` to run the full test suite
- Tests include emoji handling, message chunking, and CLI functionality

## Development Standards

- Zero Clippy warnings policy
- Full rustfmt formatting
- Comprehensive error handling
- Security: credentials are masked in logs
- Performance: optimized binary size (2.4MB)

## Troubleshooting

### Long Message Issues
If experiencing credential errors when sending long messages (>999 chars) that work fine with short messages:
- Check message chunking logic in `freemobile-api/src/chunker.rs`
- Verify rate limiting delays between chunks in `client.rs`
- Test with verbose mode: `send-sms -v -f long-file.txt`

### Common Issues
- **Credential errors**: Verify `FREEMOBILE_USER` (8 digits) and `FREEMOBILE_PASS` are set
- **File not found**: Use absolute paths or check working directory
- **Empty message**: Ensure files contain non-whitespace content
