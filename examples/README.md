# Example Data Files

This directory contains sample CSV and delimited data files for demonstrating `tbl` features.

## Files

### Business & Financial

- **sales.csv** - Quarterly sales data by product category
- **financial.csv** - Financial statement with revenue, expenses, and net income
- **pricing.csv** - Product pricing tiers

### People & Organizations

- **grades.csv** - Student grade book with multiple subjects
- **employees.csv** - Employee roster with departments and salaries
- **departments.txt** - Department information (pipe-delimited)

### Technical & Operations

- **servers.csv** - Server monitoring data with status indicators
- **benchmark.csv** - Performance benchmark comparison data
- **products.csv** - Product catalog with SKUs and inventory
- **test_results.csv** - Test suite results with scores and pass/fail status (great for conditional formatting demos)

### Data Types

- **simple.csv** - Basic 3-column table for simple demonstrations
- **matrix.csv** - Numeric matrix without headers

## Usage

See the main README.md for comprehensive examples using these files.

Quick test:

```bash
# Basic display
tbl sales.csv

# With formatting
tbl sales.csv --row-headers --column-align 2-6:right --style rounded
```

## Creating Your Own Examples

All files are plain text CSV (or delimited) format. You can:

1. Edit them directly in any text editor
2. Export from spreadsheet applications (Excel, Google Sheets, etc.)
3. Generate from scripts or databases
4. Pipe data directly: `echo "a,b,c" | tbl`

## File Format Tips

- **Headers**: First row is treated as headers (auto-detected)
- **Delimiters**: Comma (`,`), pipe (`|`), tab, semicolon (auto-detected)
- **Quotes**: Use quotes for cells containing delimiters: `"value, with comma"`
- **No headers**: Use `--no-header` flag for data-only files
