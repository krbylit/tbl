//! # tbl - A powerful CLI tool for formatting CSV data into pretty tables
//!
//! This library provides functionality for reading CSV data from various sources,
//! parsing it with auto-detection, and rendering it as beautiful formatted tables.
//!
//! ## Modules
//!
//! - `error`: Custom error types
//! - `input`: Input reading from files, stdin, and pipes
//! - `parser`: CSV parsing with auto-detection
//! - `json_parser`: JSON parsing with format detection
//! - `table`: Table rendering with configurable styles
//! - `parse_utils`: Utilities for parsing CLI arguments with range syntax

pub mod error;
pub mod input;
pub mod json_parser;
pub mod parse_utils;
pub mod parser;
pub mod table;

// Re-export commonly used types
pub use error::TblError;
pub use input::read_input;
pub use json_parser::{classify_json_format, parse_json_to_table, parse_with_depth_limit, value_to_cell_string, ColumnIndex, FormatDetector, InputFormat, JSONFormat, TableData as JsonTableData};
pub use parser::{parse_csv, ParserConfig, TableData};
pub use parse_utils::{parse_condition, Condition, ConditionOperator, ColumnIdentifier};
pub use table::{render_table, Alignment, ConditionalFormat, TableColor, TableConfig, TableStyle};
