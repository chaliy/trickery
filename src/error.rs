// User-friendly error display with icons and helpful messages.
// Design: Wraps various error types and provides clear, actionable output.

use crate::provider::ProviderError;
use std::error::Error;

/// Icons for different error categories
mod icons {
    pub const ERROR: &str = "\u{2717}"; // ✗
    pub const KEY: &str = "\u{1F511}"; // 🔑
    pub const NETWORK: &str = "\u{1F310}"; // 🌐
    pub const WARNING: &str = "\u{26A0}"; // ⚠
    pub const INFO: &str = "\u{2139}"; // ℹ
}

/// Format an error for user-friendly display
pub fn format_error(err: &(dyn Error + 'static)) -> String {
    // Try to downcast to known error types for specific handling
    if let Some(provider_err) = err.downcast_ref::<ProviderError>() {
        return format_provider_error(provider_err);
    }

    // Check for IO errors
    if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
        return format_io_error(io_err);
    }

    // Generic error fallback
    format!("{} Error: {}", icons::ERROR, err)
}

fn format_provider_error(err: &ProviderError) -> String {
    match err {
        ProviderError::MissingApiKey(key_name) => {
            format!(
                "{} Missing API Key: {}\n\n\
                 {} To fix this, set the environment variable:\n\
                 \n\
                    export {}=your_api_key_here\n\n\
                 {} You can get an API key from: https://platform.openai.com/api-keys",
                icons::KEY,
                key_name,
                icons::INFO,
                key_name,
                icons::INFO
            )
        }
        ProviderError::Http(req_err) => {
            let mut msg = format!("{} Network Error: {}", icons::NETWORK, req_err);
            if req_err.is_connect() {
                msg.push_str(&format!(
                    "\n\n{} Check your internet connection and try again.",
                    icons::INFO
                ));
            } else if req_err.is_timeout() {
                msg.push_str(&format!(
                    "\n\n{} Request timed out. The server may be busy, try again later.",
                    icons::INFO
                ));
            }
            msg
        }
        ProviderError::Api { status, message } => {
            let icon = if *status >= 500 {
                icons::NETWORK
            } else {
                icons::WARNING
            };
            let mut msg = format!("{} API Error ({}): {}", icon, status, message);

            // Add helpful hints for common error codes
            match status {
                401 => {
                    msg.push_str(&format!(
                        "\n\n{} Your API key may be invalid or expired.",
                        icons::INFO
                    ));
                }
                429 => {
                    msg.push_str(&format!(
                        "\n\n{} Rate limit exceeded. Wait a moment and try again.",
                        icons::INFO
                    ));
                }
                500..=599 => {
                    msg.push_str(&format!(
                        "\n\n{} Server error. This is likely temporary, try again later.",
                        icons::INFO
                    ));
                }
                _ => {}
            }
            msg
        }
        ProviderError::InvalidResponse(detail) => {
            format!(
                "{} Invalid Response: {}\n\n\
                 {} The API returned an unexpected response format.",
                icons::WARNING,
                detail,
                icons::INFO
            )
        }
    }
}

fn format_io_error(err: &std::io::Error) -> String {
    use std::io::ErrorKind;

    let (icon, hint) = match err.kind() {
        ErrorKind::NotFound => (icons::WARNING, "Check that the file path is correct."),
        ErrorKind::PermissionDenied => (
            icons::WARNING,
            "You don't have permission to access this file.",
        ),
        _ => (icons::ERROR, ""),
    };

    let mut msg = format!("{} File Error: {}", icon, err);
    if !hint.is_empty() {
        msg.push_str(&format!("\n\n{} {}", icons::INFO, hint));
    }
    msg
}

/// Print error to stderr in a user-friendly format
pub fn print_error(err: &(dyn Error + 'static)) {
    eprintln!("\n{}\n", format_error(err));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_missing_api_key() {
        let err = ProviderError::MissingApiKey("OPENAI_API_KEY".to_string());
        let formatted = format_error(&err);
        assert!(formatted.contains("Missing API Key"));
        assert!(formatted.contains("OPENAI_API_KEY"));
        assert!(formatted.contains("export"));
    }

    #[test]
    fn test_format_api_error_401() {
        let err = ProviderError::Api {
            status: 401,
            message: "Unauthorized".to_string(),
        };
        let formatted = format_error(&err);
        assert!(formatted.contains("401"));
        assert!(formatted.contains("invalid or expired"));
    }

    #[test]
    fn test_format_api_error_429() {
        let err = ProviderError::Api {
            status: 429,
            message: "Rate limit".to_string(),
        };
        let formatted = format_error(&err);
        assert!(formatted.contains("Rate limit exceeded"));
    }

    #[test]
    fn test_format_api_error_500() {
        let err = ProviderError::Api {
            status: 500,
            message: "Internal error".to_string(),
        };
        let formatted = format_error(&err);
        assert!(formatted.contains("Server error"));
    }

    #[test]
    fn test_format_io_not_found() {
        let err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let formatted = format_error(&err);
        assert!(formatted.contains("File Error"));
        assert!(formatted.contains("file path is correct"));
    }
}
