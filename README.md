# 📱 send-sms

[![Crates.io Version](https://img.shields.io/crates/v/send-sms?style=flat&logo=rust)](https://crates.io/crates/send-sms)
[![CI Status](https://img.shields.io/github/actions/workflow/status/davlgd/send-sms/ci.yml?branch=main&style=flat&logo=github)](https://github.com/davlgd/send-sms/actions)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg?style=flat)](./LICENSE)

A fast and user-friendly CLI to send SMS instantly from your terminal via FreeMobile API

## 📦 Installation

### For End Users (Recommended)

```bash
# Install from sources
git clone https://github.com/davlgd/send-sms
cd send-sms
make install

# Or build manually
cargo install send-sms
```

## 🚀 Usage Examples

### Basic Usage

```bash
# Simple message
send-sms -m "Hello World!"

# From a file
send-sms -f message.txt

# Interactive mode
send-sms
```

### Advanced Usage

```bash
# Pipe from command
echo "Server restarted" | send-sms

# With credentials
send-sms -u 12345678 -p your-api-key -m "Custom credentials"
```

## 📖 Input Methods

This tool supports multiple ways to provide your message:

1. 💬 **Direct Message** (`-m, --message`)
2. 📄 **File Input** (`-f, --file`)  
3. 📨 **Stdin Pipe** (auto-detected)
4. ✏️ **Interactive Mode** (default fallback)

## 📱 FreeMobile Account Setup

**You need an active FreeMobile subscription to use this tool.**

1. Go to your [FreeMobile account](https://mobile.free.fr)
2. Login with your FreeMobile credentials
3. Navigate to **Account Management** → **My Options**
4. Enable **SMS Notifications**
5. Get your **User ID** (8 digits) and generate an **API Key**

## ⚙️ Configuration Methods

1. 📁 **Environment File** (`.env`)
2. 🌍 **Environment Variables**  
3. 🔑 **CLI Arguments**
4. 💬 **Interactive Prompts** (automatic when credentials missing)

### Environment File

Create a `.env` file in your working directory:

```env
FREEMOBILE_USER=12345678
FREEMOBILE_PASS=your-api-key
```

### Environment Variables

```bash
export FREEMOBILE_USER="12345678"
export FREEMOBILE_PASS="your-api-key"
```

### CLI Arguments

```bash
send-sms -u 12345678 -p your-api-key -m "Message"
```

## 📋 Command Reference

| Option           | Alias | Description                | Example                    |
|------------------|-------|----------------------------|----------------------------|
| `--message`      | `-m`  | Direct message text        | `-m "Hello World"`         |
| `--file`         | `-f`  | Read message from file     | `-f message.txt`           |
| `--user`         | `-u`  | FreeMobile User ID         | `-u 12345678`              |
| `--pass`         | `-p`  | FreeMobile API Key         | `-p your-api-key`          |
| `--verbose`      | `-v`  | Enable verbose output      | `-v`                       |
| `--help`         | `-h`  | Show help information      | `--help`                   |
| `--version`      | `-V`  | Show version information   | `--version`                |

## 🎯 Use Cases

### DevOps & Automation

```bash
# Deployment notifications
make deploy && send-sms -m "✅ Deploy successful" || send-sms -m "❌ Deploy failed"

# System monitoring
cpu_usage=$(top -l1 | awk '/CPU usage/ {print $3}' | sed 's/%//')
[[ $cpu_usage -gt 90 ]] && send-sms -m "⚠️ High CPU: $cpu_usage%"

# Backup completion
backup.sh && send-sms -m "💾 Backup completed $(date)"
```

### Log Monitoring

```bash
# Error detection
tail -f /var/log/nginx/error.log | grep -i error | while read line; do
  send-sms -m "🚨 Nginx error: $line"
done

# Disk space alerts
df -h / | tail -1 | awk '{if($5+0 > 90) system("send-sms -m \"💾 Disk usage: "$5"\"")}'
```

## 🌟 Features

### Smart Emoji Handling

Automatically handles **146+ supported emojis** by FreeMobile:

✅ **Preserved**: ⚡ ✅ ❌ ⭐ ❤️ 🎉 🔥 💡 📱 🚀  
🔄 **Replaced**: 😀 🤖 💻 → `[]`

### Message Chunking

Long messages are automatically split:

```bash
send-sms -m "$(cat long-report.txt)"
# Sent as multiple SMS with [1/3], [2/3], [3/3] prefixes
```

### Unicode Support

```bash
send-sms -m "Café, résumé, naïf, piñata ✨"
# Accents and special characters preserved
```

## 🏗️ Project Architecture

**send-sms** is organized as a Rust workspace with two complementary crates:

### 📚 `freemobile-api` - Core Library
- Pure Rust library for FreeMobile SMS API with async/await support
- Smart emoji handling (146+ supported emojis) and automatic message chunking
- Word-boundary-aware text splitting and configurable constants
- Comprehensive error handling with typed exceptions

It's available as a standalone crate on [crates.io](https://crates.io/crates/freemobile-api) for your own projects.

### 🖥️ `send-sms-cli` - Command Line Interface  
- Multiple input methods: direct message, file input, stdin detection, interactive prompts
- Flexible configuration cascade: CLI args → env vars → .env → interactive prompts
- Smart behavior: automatic stdin detection, verbose mode, graceful error handling

```
send-sms/
├── freemobile-api/     # 📚 Reusable API library
│   ├── constants.rs    # Configurable parameters
│   ├── client.rs       # HTTP client & API integration
│   ├── sanitizer.rs    # Emoji compatibility handling
│   └── chunker.rs      # Word-aware message splitting
└── send-sms-cli/       # 🖥️ CLI interface
    ├── constants.rs    # CLI-specific limits
    ├── config.rs       # Credential management
    ├── input.rs        # Multi-source input handling
    └── main.rs         # Entry point & orchestration
```

## 🛠️ Development

### Building

```bash
# Development build
make build

# Release build
make build-release

# Run tests
make test

# Code validation
make validate
```

### Quality Standards

- **Formatted** with `rustfmt`
- **Linted** with `clippy` (zero warnings)
- **Tested** with comprehensive test suite (29 tests)
- **Documented** with rustdoc

## 📄 License

Apache 2.0 License - see [LICENSE](LICENSE) for details.

---

⭐ Found this useful? Give it a star [on GitHub](https://github.com/davlgd/send-sms) and share it with others!

Made with ❤️ for the Open Source Community
