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
//! - `table`: Table rendering with configurable styles

pub mod error;
pub mod input;
pub mod parser;
pub mod table;

// Re-export commonly used types
pub use error::TblError;
pub use input::read_input;
pub use parser::{parse_csv, ParserConfig, TableData};
pub use table::{render_table, Alignment, TableColor, TableConfig, TableStyle};
