use crate::chunker::MessageChunker;
use crate::constants::{API_URL, CHUNK_DELAY_MS, REQUEST_TIMEOUT_SECS, USER_AGENT};
use crate::error::FreeMobileError;
use crate::sanitizer::MessageSanitizer;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// FreeMobile API credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    pub user: String,
    pub pass: String,
}

impl Credentials {
    /// Create new credentials
    ///
    /// # Arguments
    ///
    /// * `user` - FreeMobile user ID (8 digits)
    /// * `pass` - FreeMobile API key
    ///
    /// # Example
    ///
    /// ```
    /// use freemobile_api::Credentials;
    ///
    /// let credentials = Credentials::new(
    ///     "12345678".to_string(),
    ///     "your-api-key".to_string()
    /// );
    /// ```
    pub fn new(user: String, pass: String) -> Self {
        Self { user, pass }
    }

    /// Check if credentials are valid (non-empty)
    pub fn is_valid(&self) -> bool {
        !self.user.trim().is_empty() && !self.pass.trim().is_empty()
    }
}

/// FreeMobile SMS API client
///
/// This client handles all communication with the FreeMobile API, including
/// message sanitization, chunking, and proper error handling.
///
/// # Example
///
/// ```rust,no_run
/// use freemobile_api::{FreeMobileClient, Credentials};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let credentials = Credentials::new(
///         "12345678".to_string(),
///         "your-api-key".to_string()
///     );
///     
///     let client = FreeMobileClient::new(credentials)?;
///     client.send("Hello World! ✅").await?;
///     
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct FreeMobileClient {
    client: Client,
    credentials: Credentials,
}

impl FreeMobileClient {
    /// Create a new FreeMobile client
    ///
    /// # Arguments
    ///
    /// * `credentials` - Valid FreeMobile API credentials
    ///
    /// # Errors
    ///
    /// Returns `FreeMobileError::InvalidCredentials` if credentials are invalid
    /// or `FreeMobileError::HttpError` if HTTP client creation fails.
    pub fn new(credentials: Credentials) -> Result<Self, FreeMobileError> {
        if !credentials.is_valid() {
            return Err(FreeMobileError::InvalidCredentials);
        }

        let client = Client::builder()
            .timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .user_agent(USER_AGENT)
            .build()
            .map_err(FreeMobileError::HttpError)?;

        Ok(Self {
            client,
            credentials,
        })
    }

    /// Send an SMS message
    ///
    /// This method automatically handles:
    /// - Message sanitization (emoji replacement)
    /// - Message chunking for long texts
    /// - Rate limiting between chunks
    ///
    /// # Arguments
    ///
    /// * `message` - The message to send (will be sanitized automatically)
    ///
    /// # Errors
    ///
    /// * `FreeMobileError::EmptyMessage` - If message is empty after trimming
    /// * `FreeMobileError::InvalidCredentials` - If API credentials are rejected  
    /// * `FreeMobileError::TooManyRequests` - If rate limit is exceeded
    /// * `FreeMobileError::HttpError` - For network-related errors
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use freemobile_api::{FreeMobileClient, Credentials};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let credentials = Credentials::new("12345678".to_string(), "key".to_string());
    /// # let client = FreeMobileClient::new(credentials)?;
    /// client.send("Hello World! ✅").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send(&self, message: &str) -> Result<(), FreeMobileError> {
        if message.trim().is_empty() {
            return Err(FreeMobileError::EmptyMessage);
        }

        let sanitized_message = MessageSanitizer::sanitize(message);
        self.send_sanitized(&sanitized_message).await
    }

    /// Send a pre-sanitized message
    ///
    /// Use this method if you want to handle sanitization yourself or send
    /// a message that has already been processed.
    ///
    /// # Arguments
    ///
    /// * `sanitized_message` - Pre-sanitized message content
    pub async fn send_sanitized(&self, sanitized_message: &str) -> Result<(), FreeMobileError> {
        if sanitized_message.trim().is_empty() {
            return Err(FreeMobileError::EmptyMessage);
        }

        let chunks = MessageChunker::chunk(sanitized_message);
        let formatted_chunks = MessageChunker::format_chunks(&chunks);

        for (index, chunk) in formatted_chunks.iter().enumerate() {
            self.send_chunk(chunk).await?;

            // Add delay between chunks to respect rate limits
            if index < formatted_chunks.len() - 1 {
                tokio::time::sleep(Duration::from_millis(CHUNK_DELAY_MS)).await;
            }
        }

        Ok(())
    }

    /// Sanitize a message without sending it
    ///
    /// This method applies the same emoji sanitization that would be applied
    /// during sending, useful for previewing changes.
    ///
    /// # Arguments
    ///
    /// * `message` - Raw message to sanitize
    ///
    /// # Returns
    ///
    /// The sanitized message with supported emojis preserved and unsupported ones replaced with []
    pub fn sanitize_message(&self, message: &str) -> String {
        MessageSanitizer::sanitize(message)
    }

    /// Send a single chunk (internal method)
    async fn send_chunk(&self, message: &str) -> Result<(), FreeMobileError> {
        let request = self.client.get(API_URL).query(&[
            ("user", &self.credentials.user),
            ("pass", &self.credentials.pass),
            ("msg", &message.to_string()),
        ]);

        let response = request.send().await.map_err(FreeMobileError::HttpError)?;

        if !response.status().is_success() {
            return Err(FreeMobileError::from_status_code(
                response.status().as_u16(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_validation() {
        let valid_creds = Credentials::new("12345678".to_string(), "abcdef123".to_string());
        assert!(valid_creds.is_valid());

        let invalid_creds = Credentials::new("".to_string(), "abcdef123".to_string());
        assert!(!invalid_creds.is_valid());
    }

    #[test]
    fn test_client_creation() {
        let valid_creds = Credentials::new("12345678".to_string(), "abcdef123".to_string());
        let client = FreeMobileClient::new(valid_creds);
        assert!(client.is_ok());

        let invalid_creds = Credentials::new("".to_string(), "abcdef123".to_string());
        let client = FreeMobileClient::new(invalid_creds);
        assert!(client.is_err());
    }

    #[test]
    fn test_sanitization_integration() {
        let creds = Credentials::new("user".to_string(), "pass".to_string());
        let client = FreeMobileClient::new(creds).unwrap();

        // Test delegation to MessageSanitizer
        let test_message = "Test ✅ supported 😀 unsupported";
        let sanitized = client.sanitize_message(test_message);
        assert_eq!(sanitized, "Test ✅ supported [] unsupported");
    }
}
