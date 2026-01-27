# Example Data Files

This directory contains sample CSV files for testing and demonstrating `tbl`.

## Files

### people.csv
Standard CSV with headers - employee information with mixed data types.

```bash
tbl examples/people.csv
```

### products.txt
Pipe-delimited file to demonstrate auto-detection of delimiters.

```bash
# Auto-detection works automatically
tbl examples/products.txt

# Or explicitly specify the delimiter
tbl examples/products.txt -d '|'
```

### matrix.csv
Numeric data without headers - useful for testing `--no-header` flag.

```bash
# Auto-detection will correctly identify no headers
tbl examples/matrix.csv

# Or force no headers
tbl examples/matrix.csv --no-header
```

## Example Commands

### Different Styles

```bash
# ASCII style
tbl examples/people.csv --style ascii

# Markdown format
tbl examples/people.csv --style markdown

# Rounded corners
tbl examples/people.csv --style rounded
```

### Alignment Options

```bash
# Right-align all columns
tbl examples/matrix.csv --align right

# Center-align with headers
tbl examples/people.csv --align center
```

### With Colors

```bash
# Green headers
tbl examples/people.csv --header-color green

# Cyan headers with ASCII style
tbl examples/products.txt --style ascii --header-color cyan
```

### Width Constraints

```bash
# Limit table width
tbl examples/people.csv --max-width 80
```

### Pipeline Usage

```bash
# Filter and display
cat examples/people.csv | grep "Engineer" | tbl

# Sort by age column
sort -t',' -k2 -n examples/people.csv | tbl
```
