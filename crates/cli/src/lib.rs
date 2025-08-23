//! send-sms CLI - Command-line interface for FreeMobile SMS API
//!
//! This crate provides a complete command-line interface for sending SMS messages
//! via the FreeMobile API. It supports multiple input methods, smart stdin detection,
//! interactive prompts, and comprehensive configuration options.

pub mod config;
pub mod constants;
pub mod input;

pub use config::Config;
pub use input::InputHandler;
