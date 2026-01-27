use anyhow::Result;
use csv::ReaderBuilder;

use crate::error::TblError;

/// Configuration for CSV parsing
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Delimiter character (None = auto-detect)
    pub delimiter: Option<char>,
    /// Whether first row contains headers (None = auto-detect)
    pub has_header: Option<bool>,
    /// Whether to trim whitespace from cells
    pub trim_whitespace: bool,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            delimiter: None,
            has_header: None,
            trim_whitespace: true,
        }
    }
}

/// Structured table data
#[derive(Debug, Clone)]
pub struct TableData {
    /// Optional header row
    pub headers: Option<Vec<String>>,
    /// Data rows
    pub rows: Vec<Vec<String>>,
}

/// Parse CSV input into structured table data
///
/// # Arguments
/// * `input` - Raw CSV string
/// * `config` - Parser configuration
///
/// # Returns
/// Parsed table data with optional headers and rows
pub fn parse_csv(input: &str, config: &ParserConfig) -> Result<TableData> {
    if input.trim().is_empty() {
        return Err(TblError::Parse("Input is empty".to_string()).into());
    }

    // Determine delimiter
    let delimiter = config
        .delimiter
        .unwrap_or_else(|| auto_detect_delimiter(input));

    // Parse CSV with determined delimiter
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .trim(csv::Trim::All)
        .flexible(true) // Allow variable number of fields
        .has_headers(false) // We'll handle headers manually
        .from_reader(input.as_bytes());

    let mut all_rows: Vec<Vec<String>> = Vec::new();
    for result in reader.records() {
        let record = result.map_err(|e| TblError::Parse(format!("CSV parsing error: {}", e)))?;
        let row: Vec<String> = record.iter().map(|s| s.to_string()).collect();
        all_rows.push(row);
    }

    if all_rows.is_empty() {
        return Err(TblError::Parse("No data found in input".to_string()).into());
    }

    // Determine if first row is header
    let has_header = config
        .has_header
        .unwrap_or_else(|| auto_detect_headers(&all_rows));

    let (headers, rows) = if has_header && !all_rows.is_empty() {
        let headers = all_rows[0].clone();
        let rows = all_rows[1..].to_vec();
        (Some(headers), rows)
    } else {
        (None, all_rows)
    };

    Ok(TableData { headers, rows })
}

/// Auto-detect the most likely delimiter character
///
/// Tries common delimiters (comma, tab, pipe, semicolon) and picks
/// the one that appears most consistently across lines.
fn auto_detect_delimiter(input: &str) -> char {
    let delimiters = [',', '\t', '|', ';'];
    let lines: Vec<&str> = input.lines().take(10).collect();

    if lines.is_empty() {
        return ','; // Default to comma
    }

    let mut best_delimiter = ',';
    let mut best_score = 0;

    for &delim in &delimiters {
        let counts: Vec<usize> = lines.iter().map(|line| line.matches(delim).count()).collect();

        // Skip if delimiter doesn't appear
        if counts.iter().all(|&c| c == 0) {
            continue;
        }

        // Calculate consistency score (lower variance = more consistent)
        let avg = counts.iter().sum::<usize>() as f64 / counts.len() as f64;
        let variance = counts
            .iter()
            .map(|&c| (c as f64 - avg).powi(2))
            .sum::<f64>()
            / counts.len() as f64;

        // Score = average count, penalized by variance
        let score = if variance < 1.0 {
            avg as usize * 100
        } else {
            (avg / variance.sqrt()) as usize
        };

        if score > best_score {
            best_score = score;
            best_delimiter = delim;
        }
    }

    best_delimiter
}

/// Auto-detect if the first row contains headers
///
/// Heuristic: If the first row has all unique strings and differs
/// in pattern from the second row (e.g., no numbers in first row
/// but numbers in second row), it's likely a header.
fn auto_detect_headers(rows: &[Vec<String>]) -> bool {
    if rows.len() < 2 {
        return false; // Can't determine with just one row
    }

    let first_row = &rows[0];
    let second_row = &rows[1];

    // Check if all values in first row are unique (common for headers)
    let unique_count = first_row.iter().collect::<std::collections::HashSet<_>>().len();
    let all_unique = unique_count == first_row.len();

    // Check if first row has no numbers but second row does
    let first_has_numbers = first_row.iter().any(|s| s.parse::<f64>().is_ok());
    let second_has_numbers = second_row.iter().any(|s| s.parse::<f64>().is_ok());

    // If first row is all unique and doesn't contain numbers while second does, it's likely a header
    if all_unique && !first_has_numbers && second_has_numbers {
        return true;
    }

    // If all values in first row are non-numeric and at least one in second is numeric
    if !first_has_numbers && second_has_numbers {
        return true;
    }

    // If both rows are all numeric, no headers
    if first_has_numbers && second_has_numbers {
        return false;
    }

    // Default to having headers if first row is all unique and non-numeric
    all_unique && !first_has_numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_with_headers() {
        let input = "name,age,city\nAlice,30,NYC\nBob,25,LA";
        let config = ParserConfig::default();
        let result = parse_csv(input, &config).unwrap();

        assert!(result.headers.is_some());
        assert_eq!(result.headers.unwrap(), vec!["name", "age", "city"]);
        assert_eq!(result.rows.len(), 2);
        assert_eq!(result.rows[0], vec!["Alice", "30", "NYC"]);
    }

    #[test]
    fn test_parse_csv_without_headers() {
        let input = "1,2,3\n4,5,6";
        let mut config = ParserConfig::default();
        config.has_header = Some(false);
        let result = parse_csv(input, &config).unwrap();

        assert!(result.headers.is_none());
        assert_eq!(result.rows.len(), 2);
        assert_eq!(result.rows[0], vec!["1", "2", "3"]);
    }

    #[test]
    fn test_auto_detect_delimiter_comma() {
        let input = "a,b,c\n1,2,3";
        assert_eq!(auto_detect_delimiter(input), ',');
    }

    #[test]
    fn test_auto_detect_delimiter_pipe() {
        let input = "a|b|c\n1|2|3";
        assert_eq!(auto_detect_delimiter(input), '|');
    }

    #[test]
    fn test_auto_detect_delimiter_tab() {
        let input = "a\tb\tc\n1\t2\t3";
        assert_eq!(auto_detect_delimiter(input), '\t');
    }

    #[test]
    fn test_auto_detect_headers_true() {
        let rows = vec![
            vec!["name".to_string(), "age".to_string(), "city".to_string()],
            vec!["Alice".to_string(), "30".to_string(), "NYC".to_string()],
        ];
        assert!(auto_detect_headers(&rows));
    }

    #[test]
    fn test_auto_detect_headers_false() {
        let rows = vec![
            vec!["1".to_string(), "2".to_string(), "3".to_string()],
            vec!["4".to_string(), "5".to_string(), "6".to_string()],
        ];
        assert!(!auto_detect_headers(&rows));
    }

    #[test]
    fn test_parse_empty_input() {
        let config = ParserConfig::default();
        let result = parse_csv("", &config);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_custom_delimiter() {
        let input = "a;b;c\n1;2;3";
        let mut config = ParserConfig::default();
        config.delimiter = Some(';');
        let result = parse_csv(input, &config).unwrap();

        assert_eq!(result.rows[0], vec!["1", "2", "3"]);
    }
}
