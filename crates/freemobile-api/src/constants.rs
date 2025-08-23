//! Configuration constants for the FreeMobile API library
//!
//! This module contains all the configurable limits, timeouts, and parameters
//! used throughout the FreeMobile API implementation.

/// Maximum message length allowed by FreeMobile API (in characters)
pub const MAX_MESSAGE_LENGTH: usize = 999;

/// Number of characters to reserve for chunk prefixes like "[99/99] "
pub const PREFIX_RESERVE_LENGTH: usize = 8;

/// HTTP request timeout for API calls
pub const REQUEST_TIMEOUT_SECS: u64 = 30;

/// Delay between consecutive chunk sends to respect rate limits
pub const CHUNK_DELAY_MS: u64 = 500;

/// User agent string for HTTP requests
pub const USER_AGENT: &str = "freemobile-api/0.1.0";

/// FreeMobile API endpoint URL
pub const API_URL: &str = "https://smsapi.free-mobile.fr/sendmsg";

/// HTTP status codes returned by FreeMobile API
pub mod status_codes {
    /// Invalid credentials - check user ID and API key
    pub const INVALID_CREDENTIALS: u16 = 400;

    /// Too many requests - rate limit exceeded
    pub const TOO_MANY_REQUESTS: u16 = 402;

    /// Access denied - check FreeMobile subscription
    pub const ACCESS_DENIED: u16 = 403;

    /// Server error - FreeMobile API internal error
    pub const SERVER_ERROR: u16 = 500;
}

/// Word boundary detection constants
pub mod word_boundary {
    /// Minimum acceptable word length for splitting
    pub const MIN_ACCEPTABLE_WORD_LENGTH: usize = 3;

    /// Minimum ratio of chunk length to consider word boundary
    /// (e.g., don't split too close to beginning: 1/3 = 33%)
    pub const MIN_BOUNDARY_RATIO: usize = 3;

    /// Minimum word recognition ratio for chunk quality validation
    pub const MIN_WORD_RECOGNITION_RATIO: f64 = 0.8;
}
