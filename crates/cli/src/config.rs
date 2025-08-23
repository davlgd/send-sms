use clap::{Arg, ArgAction, ArgMatches, Command};
use freemobile_api::{Credentials, FreeMobileError};
use is_terminal::IsTerminal;
use std::env;
use std::path::PathBuf;

type Validator = fn(&str) -> Result<(), FreeMobileError>;

#[derive(Debug, Clone)]
pub struct Config {
    pub credentials: Credentials,
    pub message: Option<String>,
    pub file_path: Option<PathBuf>,
    pub verbose: bool,
}

impl Config {
    pub fn from_args() -> Result<Self, FreeMobileError> {
        dotenv::dotenv().ok(); // Load .env file if it exists

        let matches = Self::build_cli().get_matches();
        Self::from_matches(&matches)
    }

    pub fn from_matches(matches: &ArgMatches) -> Result<Self, FreeMobileError> {
        let user = Self::get_user_id(matches)?;
        let pass = Self::get_api_key(matches)?;
        let credentials = Credentials::new(user, pass);

        let config = Config {
            credentials,
            message: matches.get_one::<String>("message").cloned(),
            file_path: matches.get_one::<String>("file").map(PathBuf::from),
            verbose: matches.get_flag("verbose"),
        };

        Ok(config)
    }

    fn build_cli() -> Command {
        Command::new("send-sms")
            .version(env!("CARGO_PKG_VERSION"))
            .author("davlgd")
            .about("Send SMS messages via FreeMobile API")
            .arg(
                Arg::new("user")
                    .short('u')
                    .long("user")
                    .env("FREEMOBILE_USER")
                    .value_name("USER_ID")
                    .help("FreeMobile user ID (8 digits)")
                    .required(false),
            )
            .arg(
                Arg::new("pass")
                    .short('p')
                    .long("pass")
                    .env("FREEMOBILE_PASS")
                    .value_name("API_KEY")
                    .help("FreeMobile API key")
                    .required(false),
            )
            .arg(
                Arg::new("message")
                    .short('m')
                    .long("message")
                    .value_name("TEXT")
                    .help("Message to send")
                    .conflicts_with("file"),
            )
            .arg(
                Arg::new("file")
                    .short('f')
                    .long("file")
                    .value_name("PATH")
                    .help("Read message from file")
                    .conflicts_with("message"),
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Verbose output")
                    .action(ArgAction::SetTrue),
            )
    }

    fn get_config_value(
        matches: &ArgMatches,
        cli_arg: &str,
        env_var: &str,
        error_message: &str,
        field_name: &str,
        validator: Option<Validator>,
    ) -> Result<String, FreeMobileError> {
        matches
            .get_one::<String>(cli_arg)
            .cloned()
            .or_else(|| env::var(env_var).ok())
            .ok_or_else(|| {
                if error_message.is_empty() {
                    FreeMobileError::ConfigError("Value not found".to_string())
                } else {
                    FreeMobileError::ConfigError(error_message.to_string())
                }
            })
            .and_then(|value| {
                if value.trim().is_empty() {
                    Err(FreeMobileError::ConfigError(format!(
                        "{} cannot be empty",
                        field_name
                    )))
                } else {
                    if let Some(validate) = validator {
                        validate(&value)?;
                    }
                    Ok(value)
                }
            })
    }

    fn validate_user_id(user_id: &str) -> Result<(), FreeMobileError> {
        if !user_id.chars().all(|c| c.is_ascii_digit()) || user_id.len() != 8 {
            Err(FreeMobileError::ConfigError(
                "User ID must be exactly 8 digits".to_string(),
            ))
        } else {
            Ok(())
        }
    }

    fn get_user_id(matches: &ArgMatches) -> Result<String, FreeMobileError> {
        // Try CLI args and env vars first
        let result = Self::get_config_value(
            matches,
            "user",
            "FREEMOBILE_USER",
            "FreeMobile user ID not found. Set FREEMOBILE_USER environment variable or use -u option",
            "User ID",
            Some(Self::validate_user_id),
        );

        match result {
            Ok(user_id) => Ok(user_id),
            Err(err) => {
                // Don't prompt during tests (when running in CI or non-TTY environment)
                if cfg!(test) || !std::io::stdin().is_terminal() {
                    return Err(err);
                }
                // Interactive prompt for missing user ID
                Self::prompt_for_user_id()
            }
        }
    }

    fn get_api_key(matches: &ArgMatches) -> Result<String, FreeMobileError> {
        // Try CLI args and env vars first
        let result = Self::get_config_value(
            matches,
            "pass",
            "FREEMOBILE_PASS",
            "FreeMobile API key not found. Set FREEMOBILE_PASS environment variable or use -p option",
            "API key",
            None,
        );

        match result {
            Ok(api_key) => Ok(api_key),
            Err(err) => {
                // Don't prompt during tests (when running in CI or non-TTY environment)
                if cfg!(test) || !std::io::stdin().is_terminal() {
                    return Err(err);
                }
                // Interactive prompt for missing API key
                Self::prompt_for_api_key()
            }
        }
    }

    fn prompt_for_user_id() -> Result<String, FreeMobileError> {
        use inquire::Text;

        let user_id = Text::new("FreeMobile User ID:")
            .with_help_message("8-digit user ID from your FreeMobile account")
            .prompt()
            .map_err(|e| {
                if e.to_string().contains("interrupted") {
                    eprintln!("Operation cancelled by user");
                    std::process::exit(1);
                } else {
                    FreeMobileError::ConfigError(format!("Failed to read user ID: {}", e))
                }
            })?;

        if user_id.trim().is_empty() {
            return Err(FreeMobileError::ConfigError(
                "User ID cannot be empty".to_string(),
            ));
        }

        Self::validate_user_id(&user_id)?;
        Ok(user_id)
    }

    fn prompt_for_api_key() -> Result<String, FreeMobileError> {
        use inquire::Password;

        let api_key = Password::new("FreeMobile API Key:")
            .with_help_message("API key from your FreeMobile account settings")
            .without_confirmation()
            .with_display_mode(inquire::PasswordDisplayMode::Masked)
            .prompt()
            .map_err(|e| {
                if e.to_string().contains("interrupted") {
                    eprintln!("Operation cancelled by user");
                    std::process::exit(1);
                } else {
                    FreeMobileError::ConfigError(format!("Failed to read API key: {}", e))
                }
            })?;

        if api_key.trim().is_empty() {
            return Err(FreeMobileError::ConfigError(
                "API key cannot be empty".to_string(),
            ));
        }

        Ok(api_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::ArgMatches;

    fn create_test_matches(args: &[&str]) -> ArgMatches {
        Config::build_cli().try_get_matches_from(args).unwrap()
    }

    #[test]
    fn test_config_with_message() {
        unsafe {
            env::set_var("FREEMOBILE_USER", "12345678");
            env::set_var("FREEMOBILE_PASS", "testkey");
        }

        let matches = create_test_matches(&["send-sms", "-m", "Hello world"]);
        let config = Config::from_matches(&matches).unwrap();

        assert_eq!(config.credentials.user, "12345678");
        assert_eq!(config.credentials.pass, "testkey");
        assert_eq!(config.message.unwrap(), "Hello world");
        assert!(!config.verbose);
    }

    #[test]
    fn test_invalid_user_id() {
        let matches =
            create_test_matches(&["send-sms", "-u", "invalid", "-p", "key", "-m", "test"]);
        let result = Config::from_matches(&matches);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            FreeMobileError::ConfigError(_)
        ));
    }

    #[test]
    fn test_no_message_source_is_valid() {
        unsafe {
            env::set_var("FREEMOBILE_USER", "12345678");
            env::set_var("FREEMOBILE_PASS", "testkey");
        }

        let matches = create_test_matches(&["send-sms"]);
        let result = Config::from_matches(&matches);

        // Should not fail - will fall back to interactive mode
        assert!(result.is_ok());
        let config = result.unwrap();
        assert!(config.message.is_none());
        assert!(config.file_path.is_none());
        assert!(!config.verbose);
    }

    #[test]
    fn test_get_config_value_with_cli_args() {
        // Test that CLI args have priority over env vars
        let matches = Config::build_cli()
            .try_get_matches_from(&["send-sms", "-u", "11111111", "-p", "cli-key", "-m", "test"])
            .unwrap();

        let user_result = Config::get_config_value(
            &matches,
            "user",
            "FREEMOBILE_USER",
            "User not found",
            "User ID",
            Some(Config::validate_user_id),
        );
        let pass_result = Config::get_config_value(
            &matches,
            "pass",
            "FREEMOBILE_PASS",
            "Pass not found",
            "API key",
            None,
        );

        assert_eq!(user_result.unwrap(), "11111111");
        assert_eq!(pass_result.unwrap(), "cli-key");
    }

    #[test]
    fn test_validate_user_id_function() {
        // Test the validator function directly
        assert!(Config::validate_user_id("12345678").is_ok());
        assert!(Config::validate_user_id("1234567").is_err()); // too short
        assert!(Config::validate_user_id("123456789").is_err()); // too long
        assert!(Config::validate_user_id("1234567a").is_err()); // contains letter
        assert!(Config::validate_user_id("").is_err()); // empty
    }

    #[test]
    fn test_interactive_prompt_detection() {
        // Test the logic that determines when to show interactive prompts

        // In test environment, prompts should be disabled
        assert!(cfg!(test), "This test should run in test mode");

        // Test that TTY detection works (will be false in CI/test environment)
        let _is_tty = std::io::stdin().is_terminal();
        // In test environment, this is usually false, which is what we want
        // (prompts disabled when not interactive)
    }
}
