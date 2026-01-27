use anyhow::Result;
use atty::Stream;
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

use crate::error::TblError;

/// Read input from either a file or stdin
///
/// # Arguments
/// * `path` - Optional file path. If None or "-", reads from stdin
///
/// # Returns
/// The input content as a String
///
/// # Errors
/// Returns TblError::Input if file cannot be read or contains invalid UTF-8
pub fn read_input(path: Option<PathBuf>) -> Result<String> {
    match path {
        // No path provided - read from stdin
        None => read_stdin(),
        // Explicit stdin marker or file path
        Some(p) if p.to_str() == Some("-") => read_stdin(),
        // Read from file
        Some(path) => read_file(&path),
    }
}

/// Read input from stdin
///
/// Works with pipes and redirects. If stdin is a terminal (interactive),
/// returns an error suggesting the user provide a file or pipe input.
fn read_stdin() -> Result<String> {
    // Check if stdin is a terminal (interactive) rather than piped/redirected
    if atty::is(Stream::Stdin) {
        return Err(TblError::Input(
            "No input provided. Please provide a CSV file or pipe data to stdin.

Examples:
  tbl data.csv
  echo \"a,b,c\" | tbl
  cat file.csv | tbl

Use --help for more information."
                .to_string(),
        )
        .into());
    }

    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .map_err(|e| TblError::Input(format!("Failed to read from stdin: {}", e)))?;

    if buffer.is_empty() {
        return Err(TblError::Input("No input provided".to_string()).into());
    }

    Ok(buffer)
}

/// Read input from a file
fn read_file(path: &PathBuf) -> Result<String> {
    // Check if file exists
    if !path.exists() {
        return Err(TblError::Input(format!(
            "File not found: {}",
            path.display()
        ))
        .into());
    }

    // Check if it's a file (not a directory)
    if !path.is_file() {
        return Err(TblError::Input(format!(
            "Path is not a file: {}",
            path.display()
        ))
        .into());
    }

    // Read file contents
    fs::read_to_string(path).map_err(|e| {
        TblError::Input(format!("Failed to read file {}: {}", path.display(), e)).into()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_read_file_success() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "test,data").unwrap();

        let result = read_file(&temp_file.path().to_path_buf());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "test,data");
    }

    #[test]
    fn test_read_file_not_found() {
        let result = read_file(&PathBuf::from("/nonexistent/file.csv"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("File not found"));
    }

    #[test]
    fn test_read_file_directory() {
        let result = read_file(&PathBuf::from("/tmp"));
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not a file"));
    }
}
