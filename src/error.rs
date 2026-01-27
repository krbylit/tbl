use std::fmt;

/// Custom error types for better user-facing error messages
#[derive(Debug)]
pub enum TblError {
    /// Input-related errors (file not found, read errors, etc.)
    Input(String),
    /// CSV parsing errors (malformed CSV, encoding issues)
    Parse(String),
    /// Configuration errors (conflicting flags, invalid options)
    Config(String),
}

impl fmt::Display for TblError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TblError::Input(msg) => write!(f, "Input error: {}", msg),
            TblError::Parse(msg) => write!(f, "Parse error: {}", msg),
            TblError::Config(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl std::error::Error for TblError {}
