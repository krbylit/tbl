use anyhow::{Context, Result};
use indexmap::IndexMap;
use serde_json::Value;

/// Represents the three supported JSON table formats
#[derive(Debug, Clone)]
pub enum JSONFormat {
    /// Array of objects: [{"name": "Alice", "age": 30}, ...]
    /// Most common format for API responses and exports
    ArrayOfObjects(Vec<serde_json::Map<String, Value>>),

    /// Single object: {"name": "Alice", "age": 30}
    /// Used for config files or single-record inspection
    SingleObject(serde_json::Map<String, Value>),

    /// Columnar format: {"name": ["Alice", "Bob"], "age": [30, 25]}
    /// Data science tools (pandas, numpy) output format
    ColumnarObject(serde_json::Map<String, Value>),
}

/// Tabular data structure (rows and columns)
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

const MAX_DEPTH: usize = 128;

/// Parse JSON with depth limit enforcement
///
/// Note: serde_json has a built-in recursion limit of 128, which aligns
/// with our MAX_DEPTH. We rely on serde_json's limit for parsing safety,
/// then validate with our own check_depth for consistency.
pub fn parse_with_depth_limit(input: &str) -> Result<Value> {
    let value: Value = serde_json::from_str(input)
        .context("Failed to parse JSON input")?;

    // Validate depth after parsing
    check_depth(&value, 0)?;

    Ok(value)
}

/// Recursively check JSON nesting depth
fn check_depth(value: &Value, current_depth: usize) -> Result<()> {
    if current_depth > MAX_DEPTH {
        anyhow::bail!("JSON nesting depth exceeds maximum ({} levels)", MAX_DEPTH);
    }

    match value {
        Value::Object(map) => {
            for v in map.values() {
                check_depth(v, current_depth + 1)?;
            }
        }
        Value::Array(arr) => {
            for v in arr {
                check_depth(v, current_depth + 1)?;
            }
        }
        _ => {} // Primitives don't increase depth
    }

    Ok(())
}

/// Input format detection
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputFormat {
    Json,
    Csv,
}

/// Detect input format based on first non-whitespace character
pub struct FormatDetector;

impl FormatDetector {
    pub fn detect(input: &str) -> InputFormat {
        match input.trim_start().chars().next() {
            Some('[') | Some('{') => InputFormat::Json,
            _ => InputFormat::Csv,
        }
    }
}

/// Track column order using first-occurrence ordering
pub struct ColumnIndex {
    map: IndexMap<String, usize>,
}

impl ColumnIndex {
    pub fn new() -> Self {
        ColumnIndex {
            map: IndexMap::new(),
        }
    }

    /// Insert column if not present, return index
    pub fn insert(&mut self, key: String) -> usize {
        let next_index = self.map.len();
        *self.map.entry(key).or_insert(next_index)
    }

    /// Get all column names in first-occurrence order
    pub fn keys(&self) -> Vec<String> {
        self.map.keys().cloned().collect()
    }

    /// Get column index
    pub fn index_of(&self, key: &str) -> Option<usize> {
        self.map.get(key).copied()
    }
}

/// Convert JSON value to table cell string
pub fn value_to_cell_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(), // Empty cell for null
        Value::Array(_) | Value::Object(_) => {
            // Stringify nested structures as compact JSON
            serde_json::to_string(value).unwrap_or_else(|_| String::new())
        }
    }
}

/// Classify JSON value into one of the three supported table formats
pub fn classify_json_format(value: Value) -> Result<JSONFormat> {
    match value {
        Value::Array(arr) => {
            // Array of objects format
            let mut objects = Vec::new();
            for item in arr {
                match item {
                    Value::Object(obj) => objects.push(obj),
                    _ => anyhow::bail!("Array contains non-object element. All elements must be objects."),
                }
            }
            Ok(JSONFormat::ArrayOfObjects(objects))
        }
        Value::Object(obj) => {
            // Distinguish between SingleObject and ColumnarObject
            // Columnar: all values are arrays of same length
            // SingleObject: at least one value is not an array

            if obj.is_empty() {
                return Ok(JSONFormat::SingleObject(obj));
            }

            let mut is_columnar = true;
            let mut first_len: Option<usize> = None;

            for val in obj.values() {
                match val {
                    Value::Array(arr) => {
                        let len = arr.len();
                        match first_len {
                            None => first_len = Some(len),
                            Some(expected_len) => {
                                if len != expected_len {
                                    is_columnar = false;
                                    break;
                                }
                            }
                        }
                    }
                    _ => {
                        is_columnar = false;
                        break;
                    }
                }
            }

            if is_columnar && first_len.is_some() {
                Ok(JSONFormat::ColumnarObject(obj))
            } else {
                Ok(JSONFormat::SingleObject(obj))
            }
        }
        _ => anyhow::bail!("Input must be a JSON object or array"),
    }
}

/// Collect column headers from array of objects in first-occurrence order
fn collect_column_headers(objects: &[serde_json::Map<String, Value>]) -> Vec<String> {
    let mut index = ColumnIndex::new();

    for obj in objects {
        for key in obj.keys() {
            index.insert(key.clone());
        }
    }

    index.keys()
}

/// Convert array of objects to table data
fn convert_array_of_objects_to_table(objects: Vec<serde_json::Map<String, Value>>) -> TableData {
    if objects.is_empty() {
        return TableData {
            headers: vec![],
            rows: vec![],
        };
    }

    let headers = collect_column_headers(&objects);
    let mut rows = Vec::new();

    for obj in objects {
        let mut row = Vec::new();
        for header in &headers {
            let cell_value = obj.get(header)
                .map(value_to_cell_string)
                .unwrap_or_else(String::new);
            row.push(cell_value);
        }
        rows.push(row);
    }

    TableData { headers, rows }
}

/// Parse JSON value to table data
pub fn parse_json_to_table(value: Value) -> Result<TableData> {
    let format = classify_json_format(value)?;

    match format {
        JSONFormat::ArrayOfObjects(objects) => {
            Ok(convert_array_of_objects_to_table(objects))
        }
        JSONFormat::SingleObject(_obj) => {
            // TODO: Implement in User Story 2
            anyhow::bail!("Single object format not yet implemented")
        }
        JSONFormat::ColumnarObject(_obj) => {
            // TODO: Implement in User Story 3
            anyhow::bail!("Columnar format not yet implemented")
        }
    }
}
