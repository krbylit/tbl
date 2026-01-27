/// Utilities for parsing CLI arguments with range and multi-target syntax
use anyhow::Result;

/// Condition operators for conditional formatting
#[derive(Debug, Clone, PartialEq)]
pub enum ConditionOperator {
    // Numeric comparisons
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,

    // String matching
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
}

/// Column identifier (by index or name)
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnIdentifier {
    Index(usize),      // 0-based column index
    Name(String),      // Column name from header
}

/// Condition for conditional formatting
#[derive(Debug, Clone)]
pub struct Condition {
    pub column: ColumnIdentifier,
    pub operator: ConditionOperator,
    pub value: String,
}

/// Parse a target specification that may contain:
/// - Single values: "2"
/// - Lists: "2,3,4"
/// - Ranges: "2-4"
/// - Mixed: "1,3-5,7"
///
/// Returns a Vec of all target indices (0-based internally, but input is 1-based)
pub fn parse_targets(s: &str, name: &str) -> Result<Vec<usize>, String> {
    let mut targets = Vec::new();

    for part in s.split(',') {
        if part.contains('-') {
            // Range syntax: "2-4"
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() != 2 {
                return Err(format!(
                    "Invalid range in {}: '{}'. Expected format: START-END (e.g., 2-4)",
                    name, part
                ));
            }

            let start = range_parts[0]
                .parse::<usize>()
                .map_err(|_| format!("Invalid start in range '{}': '{}'", part, range_parts[0]))?;

            let end = range_parts[1]
                .parse::<usize>()
                .map_err(|_| format!("Invalid end in range '{}': '{}'", part, range_parts[1]))?;

            if start == 0 || end == 0 {
                return Err(format!(
                    "{} numbers start at 1 (not 0). Range '{}' is invalid.",
                    name, part
                ));
            }

            if start > end {
                return Err(format!(
                    "Invalid range '{}': start ({}) must be <= end ({})",
                    part, start, end
                ));
            }

            // Convert to 0-based and add all values in range
            for i in start..=end {
                targets.push(i - 1);
            }
        } else {
            // Single value
            let val = part
                .parse::<usize>()
                .map_err(|_| format!("Invalid {} number: '{}'", name, part))?;

            if val == 0 {
                return Err(format!(
                    "{} numbers start at 1 (not 0)",
                    name
                ));
            }

            targets.push(val - 1); // Convert to 0-based
        }
    }

    Ok(targets)
}

/// Parse cell coordinates, which can be:
/// - List: "1,1,2,2,3,3" (pairs of col,row)
/// - Range: "1,1-3,3" (from col1,row1 to col2,row2)
///
/// Returns either a list of cell coordinates or a range
#[derive(Debug, Clone)]
pub enum CellTarget {
    List(Vec<(usize, usize)>), // 0-based coordinates
    Range {
        start_col: usize,
        start_row: usize,
        end_col: usize,
        end_row: usize,
    },
}

impl CellTarget {
    /// Get all cells covered by this target (0-based)
    pub fn cells(&self) -> Vec<(usize, usize)> {
        match self {
            CellTarget::List(cells) => cells.clone(),
            CellTarget::Range {
                start_col,
                start_row,
                end_col,
                end_row,
            } => {
                let mut cells = Vec::new();
                for row in *start_row..=*end_row {
                    for col in *start_col..=*end_col {
                        cells.push((col, row));
                    }
                }
                cells
            }
        }
    }
}

pub fn parse_cell_targets(s: &str) -> Result<CellTarget, String> {
    // Check if it's a range (contains hyphen before the colon)
    if s.contains('-') {
        // Range syntax: "1,1-3,3"
        let coords_parts: Vec<&str> = s.split('-').collect();
        if coords_parts.len() != 2 {
            return Err(format!(
                "Invalid cell range: '{}'. Expected format: COL,ROW-COL,ROW (e.g., 1,1-3,3)",
                s
            ));
        }

        let start_parts: Vec<&str> = coords_parts[0].split(',').collect();
        let end_parts: Vec<&str> = coords_parts[1].split(',').collect();

        if start_parts.len() != 2 || end_parts.len() != 2 {
            return Err(format!(
                "Invalid cell range: '{}'. Each coordinate must be COL,ROW",
                s
            ));
        }

        let start_col = start_parts[0]
            .parse::<usize>()
            .map_err(|_| format!("Invalid column number: '{}'", start_parts[0]))?;
        let start_row = start_parts[1]
            .parse::<usize>()
            .map_err(|_| format!("Invalid row number: '{}'", start_parts[1]))?;
        let end_col = end_parts[0]
            .parse::<usize>()
            .map_err(|_| format!("Invalid column number: '{}'", end_parts[0]))?;
        let end_row = end_parts[1]
            .parse::<usize>()
            .map_err(|_| format!("Invalid row number: '{}'", end_parts[1]))?;

        if start_col == 0 || start_row == 0 || end_col == 0 || end_row == 0 {
            return Err("Cell coordinates start at 1 (not 0)".to_string());
        }

        if start_col > end_col || start_row > end_row {
            return Err(format!(
                "Invalid range: start ({},{}) must be <= end ({},{})",
                start_col, start_row, end_col, end_row
            ));
        }

        Ok(CellTarget::Range {
            start_col: start_col - 1, // Convert to 0-based
            start_row: start_row - 1,
            end_col: end_col - 1,
            end_row: end_row - 1,
        })
    } else {
        // List syntax: "1,1,2,2,3,3"
        let nums: Vec<&str> = s.split(',').collect();

        if nums.len() % 2 != 0 {
            return Err(format!(
                "Invalid cell list: '{}'. Must have even number of coordinates (col,row pairs)",
                s
            ));
        }

        let mut cells = Vec::new();
        for i in (0..nums.len()).step_by(2) {
            let col = nums[i]
                .parse::<usize>()
                .map_err(|_| format!("Invalid column number: '{}'", nums[i]))?;
            let row = nums[i + 1]
                .parse::<usize>()
                .map_err(|_| format!("Invalid row number: '{}'", nums[i + 1]))?;

            if col == 0 || row == 0 {
                return Err("Cell coordinates start at 1 (not 0)".to_string());
            }

            cells.push((col - 1, row - 1)); // Convert to 0-based
        }

        Ok(CellTarget::List(cells))
    }
}

/// Parse a condition string (format: "COL<VALUE", "COL=VALUE", "NameContains:text")
/// Examples:
///   "3<0" - Column 3 less than 0
///   "Status=Error" - Column "Status" equals "Error"
///   "2>=90" - Column 2 greater than or equal to 90
///   "Messagecontains:fail" - Column "Message" contains "fail"
pub fn parse_condition(s: &str) -> Result<Condition, String> {
    // Try to parse operators in order of length (longest first to avoid substring matches)
    let operators = [
        ("<=", ConditionOperator::LessThanOrEqual),
        (">=", ConditionOperator::GreaterThanOrEqual),
        ("!=", ConditionOperator::NotEqual),
        ("!contains:", ConditionOperator::NotContains),
        ("contains:", ConditionOperator::Contains),
        ("starts:", ConditionOperator::StartsWith),
        ("ends:", ConditionOperator::EndsWith),
        ("<", ConditionOperator::LessThan),
        (">", ConditionOperator::GreaterThan),
        ("=", ConditionOperator::Equal),
    ];

    for (op_str, operator) in operators.iter() {
        if let Some(pos) = s.find(op_str) {
            let column_str = &s[..pos];
            let value = s[pos + op_str.len()..].to_string();

            // Try to parse as column index (1-based)
            let column = if let Ok(idx) = column_str.parse::<usize>() {
                if idx == 0 {
                    return Err("Column numbers start at 1 (not 0)".to_string());
                }
                ColumnIdentifier::Index(idx - 1) // Convert to 0-based
            } else {
                // Treat as column name
                ColumnIdentifier::Name(column_str.to_string())
            };

            return Ok(Condition {
                column,
                operator: operator.clone(),
                value,
            });
        }
    }

    Err(format!(
        "Invalid condition: '{}'. Expected format: COL<OP>VALUE (e.g., 3<0, Status=Error, 2>=90)",
        s
    ))
}

impl Condition {
    /// Evaluate if a cell value matches this condition
    pub fn matches(&self, value: &str) -> bool {
        match &self.operator {
            // Numeric comparisons - try to parse both as numbers
            ConditionOperator::LessThan => {
                if let (Ok(v), Ok(threshold)) = (value.parse::<f64>(), self.value.parse::<f64>()) {
                    v < threshold
                } else {
                    false
                }
            }
            ConditionOperator::LessThanOrEqual => {
                if let (Ok(v), Ok(threshold)) = (value.parse::<f64>(), self.value.parse::<f64>()) {
                    v <= threshold
                } else {
                    false
                }
            }
            ConditionOperator::GreaterThan => {
                if let (Ok(v), Ok(threshold)) = (value.parse::<f64>(), self.value.parse::<f64>()) {
                    v > threshold
                } else {
                    false
                }
            }
            ConditionOperator::GreaterThanOrEqual => {
                if let (Ok(v), Ok(threshold)) = (value.parse::<f64>(), self.value.parse::<f64>()) {
                    v >= threshold
                } else {
                    false
                }
            }
            ConditionOperator::Equal => {
                // Try numeric first, fall back to string
                if let (Ok(v), Ok(threshold)) = (value.parse::<f64>(), self.value.parse::<f64>()) {
                    (v - threshold).abs() < f64::EPSILON
                } else {
                    value == self.value
                }
            }
            ConditionOperator::NotEqual => {
                // Try numeric first, fall back to string
                if let (Ok(v), Ok(threshold)) = (value.parse::<f64>(), self.value.parse::<f64>()) {
                    (v - threshold).abs() >= f64::EPSILON
                } else {
                    value != self.value
                }
            }

            // String matching
            ConditionOperator::Contains => value.contains(&self.value),
            ConditionOperator::NotContains => !value.contains(&self.value),
            ConditionOperator::StartsWith => value.starts_with(&self.value),
            ConditionOperator::EndsWith => value.ends_with(&self.value),
        }
    }

    /// Get the column index for this condition, resolving names from headers
    pub fn get_column_index(&self, headers: Option<&[String]>) -> Option<usize> {
        match &self.column {
            ColumnIdentifier::Index(idx) => Some(*idx),
            ColumnIdentifier::Name(name) => {
                // Find column by name in headers
                headers.and_then(|h| h.iter().position(|header| header == name))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_targets_single() {
        let result = parse_targets("2", "column").unwrap();
        assert_eq!(result, vec![1]); // 1-based input, 0-based output
    }

    #[test]
    fn test_parse_targets_list() {
        let result = parse_targets("2,3,4", "column").unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_targets_range() {
        let result = parse_targets("2-4", "column").unwrap();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_targets_mixed() {
        let result = parse_targets("1,3-5,7", "column").unwrap();
        assert_eq!(result, vec![0, 2, 3, 4, 6]);
    }

    #[test]
    fn test_parse_targets_zero_error() {
        let result = parse_targets("0", "column");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_cell_targets_list() {
        let result = parse_cell_targets("1,1,2,2").unwrap();
        match result {
            CellTarget::List(cells) => {
                assert_eq!(cells, vec![(0, 0), (1, 1)]);
            }
            _ => panic!("Expected List"),
        }
    }

    #[test]
    fn test_parse_cell_targets_range() {
        let result = parse_cell_targets("1,1-2,2").unwrap();
        match result {
            CellTarget::Range {
                start_col,
                start_row,
                end_col,
                end_row,
            } => {
                assert_eq!(start_col, 0);
                assert_eq!(start_row, 0);
                assert_eq!(end_col, 1);
                assert_eq!(end_row, 1);
            }
            _ => panic!("Expected Range"),
        }
    }

    #[test]
    fn test_cell_target_cells_range() {
        let target = CellTarget::Range {
            start_col: 0,
            start_row: 0,
            end_col: 1,
            end_row: 1,
        };
        let cells = target.cells();
        assert_eq!(cells, vec![(0, 0), (1, 0), (0, 1), (1, 1)]);
    }

    #[test]
    fn test_parse_condition_numeric() {
        let cond = parse_condition("3<0").unwrap();
        assert!(matches!(cond.column, ColumnIdentifier::Index(2))); // 3 -> 2 (0-based)
        assert!(matches!(cond.operator, ConditionOperator::LessThan));
        assert_eq!(cond.value, "0");
    }

    #[test]
    fn test_parse_condition_string_column() {
        let cond = parse_condition("Status=Error").unwrap();
        assert!(matches!(cond.column, ColumnIdentifier::Name(ref name) if name == "Status"));
        assert!(matches!(cond.operator, ConditionOperator::Equal));
        assert_eq!(cond.value, "Error");
    }

    #[test]
    fn test_parse_condition_contains() {
        let cond = parse_condition("Messagecontains:fail").unwrap();
        assert!(matches!(cond.column, ColumnIdentifier::Name(ref name) if name == "Message"));
        assert!(matches!(cond.operator, ConditionOperator::Contains));
        assert_eq!(cond.value, "fail");
    }

    #[test]
    fn test_condition_matches_numeric() {
        let cond = Condition {
            column: ColumnIdentifier::Index(0),
            operator: ConditionOperator::LessThan,
            value: "50".to_string(),
        };
        assert!(cond.matches("25"));
        assert!(!cond.matches("75"));
    }

    #[test]
    fn test_condition_matches_string() {
        let cond = Condition {
            column: ColumnIdentifier::Name("Status".to_string()),
            operator: ConditionOperator::Equal,
            value: "Error".to_string(),
        };
        assert!(cond.matches("Error"));
        assert!(!cond.matches("OK"));
    }

    #[test]
    fn test_condition_matches_contains() {
        let cond = Condition {
            column: ColumnIdentifier::Index(0),
            operator: ConditionOperator::Contains,
            value: "warn".to_string(),
        };
        assert!(cond.matches("warning: file not found"));
        assert!(!cond.matches("error: bad input"));
    }
}
