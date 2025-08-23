//! Configuration constants for the send-sms CLI
//!
//! This module contains all the configurable limits and parameters
//! used throughout the application.

/// Maximum message length for input validation
/// This is a reasonable limit to prevent extremely large messages
/// before they are processed by the chunking system
pub const MAX_MESSAGE_LENGTH: usize = 5000;

/// Preview length for message display in verbose mode  
/// Shows first N characters of the message for user feedback
pub const MESSAGE_PREVIEW_LENGTH: usize = 100;
