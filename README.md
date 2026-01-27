# tbl - Pretty Table Formatter for CSV Data

A powerful, scriptable CLI tool for formatting CSV data into beautiful tables. Built with Rust for speed and reliability.

## Features

- 📥 **Flexible Input**: Read from files or stdin
- 🔍 **Auto-Detection**: Automatically detects delimiters and headers
- 🎨 **Multiple Styles**: 6 built-in table styles (ASCII, Unicode, Markdown, and more)
- ⚙️ **Highly Configurable**: Extensive options for alignment, borders, colors, and layout
- 🎯 **Conditional Formatting**: Automatically color and style cells based on their values
- 🚀 **Fast & Efficient**: Written in Rust for optimal performance
- 📝 **Script-Friendly**: Designed for use in pipelines and automation

## Installation

### From Source

```bash
# Clone or navigate to the project directory
git clone <repository-url>
cd tbl

# Build and install
cargo install --path .

# Or just build
cargo build --release
# Binary will be at: target/release/tbl
```

### Shell Completions

Generate shell completions for your shell:

```bash
# Bash
tbl completions bash > ~/.local/share/bash-completion/completions/tbl
source ~/.bashrc

# Zsh
tbl completions zsh > ~/.zsh/completions/_tbl
# Add to .zshrc: fpath=(~/.zsh/completions $fpath)
source ~/.zshrc

# Fish
tbl completions fish > ~/.config/fish/completions/tbl.fish
source ~/.config/fish/config.fish

# PowerShell
tbl completions powershell > tbl.ps1
. ./tbl.ps1
```

Completions provide tab-completion for:

- All flag names (`--header-color`, `--style`, etc.)
- Enum values (styles, colors, alignments)
- File paths

## Quick Start

```bash
# Basic usage with pipe
echo -e "name,age,city\nAlice,30,NYC\nBob,25,LA" | tbl

# From a file
tbl data.csv

# With custom style
cat data.csv | tbl --style ascii

# Right-aligned with markdown output
tbl data.csv --style markdown --align right
```

> **💡 Try it yourself!** The `examples/` directory contains sample data files. See [Real-World Examples](#real-world-examples) for comprehensive demonstrations of all features.

## Usage

```
tbl [OPTIONS] [FILE]
```

### Arguments

- `[FILE]` - Input CSV file (use `-` for stdin, or omit for stdin by default)

### Input Format Options

| Option                   | Description                                 |
| ------------------------ | ------------------------------------------- |
| `--csv`                  | Force CSV format (comma-separated, default) |
| `-d, --delimiter <CHAR>` | Use custom delimiter character              |
| `--header`               | Force first row as headers                  |
| `--no-header`            | Force first row as data (no headers)        |
| `--no-trim`              | Don't trim whitespace from cells            |

> **Note**: By default, `tbl` auto-detects delimiters and headers intelligently.

### Table Style Options

| Option                | Description                             |
| --------------------- | --------------------------------------- |
| `-s, --style <STYLE>` | Table style preset (default: `unicode`) |

**Available Styles**:

- `ascii` - ASCII characters only (`+-|`)
- `unicode` - Unicode box drawing (default)
- `markdown` - Markdown-compatible tables
- `rounded` - Rounded corners with Unicode
- `sharp` - Sharp ASCII style
- `dots` - Dotted borders

### Layout Options

| Option                    | Description                       |
| ------------------------- | --------------------------------- |
| `-w, --max-width <WIDTH>` | Maximum table width in characters |
| `--padding <NUM>`         | Horizontal padding (default: 1)   |

### Alignment Options

| Option                   | Description                                                   |
| ------------------------ | ------------------------------------------------------------- |
| `-a, --align <ALIGN>`    | Column alignment: `left`, `center`, `right` (default: `left`) |
| `--header-align <ALIGN>` | Header alignment (default: same as columns)                   |

### Border Options

| Option                  | Description                  |
| ----------------------- | ---------------------------- |
| `--no-outer-border`     | Remove outer borders         |
| `--no-column-border`    | Remove column separators     |
| `--no-header-separator` | Remove header separator line |

### Color Options

| Option                      | Description                                  |
| --------------------------- | -------------------------------------------- |
| `--header-color <COLOR>`    | Header text color                            |
| `--header-bg-color <COLOR>` | Header background color                      |
| `--alt-row-bg <C1>,<C2>`    | Alternating row backgrounds (zebra striping) |

**Available Colors**: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, and `bright-*` variants

### Text Attributes

| Option               | Description                 |
| -------------------- | --------------------------- |
| `--header-bold`      | Make header text bold       |
| `--header-underline` | Make header text underlined |

### Column Options

All column options support **range syntax** (`2-4`) and **multi-target syntax** (`2,3,5`).

| Option                          | Description                      |
| ------------------------------- | -------------------------------- |
| `--column-align <COLS>:<ALIGN>` | Set alignment for columns        |
| `--column-color <COLS>:<COLOR>` | Set text color for columns       |
| `--column-bg <COLS>:<COLOR>`    | Set background color for columns |
| `--column-bold <COLS>`          | Make columns bold                |

**Examples**:

- `--column-align 2:right` (single column)
- `--column-align 2-4:right` (range: columns 2, 3, 4)
- `--column-color 2,3,5:blue` (multiple: columns 2, 3, and 5)

### Row Options

All row options support **range syntax** (`2-4`) and **multi-target syntax** (`1,3,5`).
Row numbers are 1-based (first data row = 1, not counting headers).

| Option                       | Description                   |
| ---------------------------- | ----------------------------- |
| `--row-color <ROWS>:<COLOR>` | Set text color for rows       |
| `--row-bg <ROWS>:<COLOR>`    | Set background color for rows |
| `--row-bold <ROWS>`          | Make rows bold                |

**Examples**:

- `--row-color 1:red` (highlight first row)
- `--row-bg 2-4:bright-black` (shade rows 2-4)

### Cell Options

Cell options support **list syntax** (`1,1,2,2`) and **range syntax** (`1,1-3,3`).
Cell coordinates are `COL,ROW` pairs (both 1-based).

| Option                          | Description                       |
| ------------------------------- | --------------------------------- |
| `--cell-color <COORDS>:<COLOR>` | Set text color for specific cells |
| `--cell-bg <COORDS>:<COLOR>`    | Set background color for cells    |
| `--cell-bold <COORDS>`          | Make cells bold                   |

**Examples**:

- `--cell-color 1,1:red` (single cell at column 1, row 1)
- `--cell-bg 1,1-3,3:yellow` (rectangular range from 1,1 to 3,3)
- `--cell-bold 1,1,2,2,3,3` (list of cells)

### Row Headers (First Column)

Enable row headers to treat the first column as row labels (like a spreadsheet).

| Option                          | Description                      |
| ------------------------------- | -------------------------------- |
| `--row-headers`                 | Enable row header mode           |
| `--row-header-color <COLOR>`    | Text color for row headers       |
| `--row-header-bg-color <COLOR>` | Background color for row headers |
| `--row-header-bold`             | Make row headers bold            |
| `--row-header-underline`        | Underline row headers            |
| `--row-header-align <ALIGN>`    | Alignment for row headers        |

### Corner Cell Options

When both column headers (first row) and row headers (first column) are present, the top-left cell is the "corner cell".

| Option                      | Description                      |
| --------------------------- | -------------------------------- |
| `--corner-text <TEXT>`      | Custom text for corner cell      |
| `--corner-color <COLOR>`    | Text color for corner cell       |
| `--corner-bg-color <COLOR>` | Background color for corner cell |

## Examples

### Basic CSV with Headers

```bash
echo -e "name,age,city\nAlice,30,NYC\nBob,25,LA" | tbl
```

```
┌───────┬─────┬──────┐
│ name  ┆ age ┆ city │
╞═══════╪═════╪══════╡
│ Alice ┆ 30  ┆ NYC  │
├╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌┤
│ Bob   ┆ 25  ┆ LA   │
└───────┴─────┴──────┘
```

### ASCII Style

```bash
tbl data.csv --style ascii
```

```
+-------+-----+------+
| name  | age | city |
+====================+
| Alice | 30  | NYC  |
+-------+-----+------+
| Bob   | 25  | LA   |
+-------+-----+------+
```

### Markdown Format

```bash
tbl data.csv --style markdown
```

Perfect for documentation:

```
┌───────────────────┐
│ name   age   city │
╞═══════════════════╡
│ Alice  30    NYC  │
│ Bob    25    LA   │
└───────────────────┘
```

### Pipe-Delimited Input

Auto-detection works seamlessly:

```bash
echo -e "fruit|color|taste\napple|red|sweet" | tbl
```

```
┌───────┬───────┬───────┐
│ fruit ┆ color ┆ taste │
╞═══════╪═══════╪═══════╡
│ apple ┆ red   ┆ sweet │
└───────┴───────┴───────┘
```

### Right-Aligned Numbers

```bash
cat prices.csv | tbl --align right
```

```
┌─────────┬───────┬─────┐
│ Product ┆ Price ┆ Qty │
╞═════════╪═══════╪═════╡
│   Apple ┆  1.50 ┆  10 │
├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌┼╌╌╌╌╌┤
│  Banana ┆  0.75 ┆  25 │
└─────────┴───────┴─────┘
```

### No Headers

```bash
echo -e "1,2,3\n4,5,6" | tbl --no-header
```

```
┌───┬───┬───┐
│ 1 ┆ 2 ┆ 3 │
├╌╌╌┼╌╌╌┼╌╌╌┤
│ 4 ┆ 5 ┆ 6 │
└───┴───┴───┘
```

### Custom Delimiter

```bash
tbl data.txt -d ';' --style rounded
```

### With Header Styling

```bash
# Bold headers with background
tbl data.csv --header-bold --header-bg-color black --header-color cyan
```

### Column-Specific Alignment

```bash
# Right-align numeric columns
tbl financial.csv --column-align 2:right --column-align 3:right --column-align 4:right
```

### Zebra Striping (Alternating Row Colors)

```bash
# Subtle alternating backgrounds for readability
tbl data.csv --alt-row-bg white,bright-black
```

### Advanced Combination

```bash
# Professional report styling
tbl report.csv \
  --header-bold \
  --header-bg-color black \
  --header-color cyan \
  --column-align 1:left \
  --column-align 2:right \
  --column-align 3:right \
  --alt-row-bg white,bright-black \
  --style rounded
```

### Column Formatting with Ranges

```bash
# Right-align all numeric columns at once
tbl data.csv --column-align 2-5:right --column-bold 2-5

# Color-code multiple columns
tbl sales.csv \
  --column-color 2:green \
  --column-color 3-4:yellow \
  --column-bg 5:bright-black
```

### Row Highlighting

```bash
# Highlight header row and first data row
echo -e "Name,Score\nAlice,95\nBob,78\nCarol,88" | tbl \
  --row-color 1:bright-blue \
  --row-bold 1

# Shade alternate sections
tbl report.csv --row-bg 1-3:white --row-bg 4-6:bright-black
```

### Cell-Level Formatting

```bash
# Highlight specific cells
tbl data.csv \
  --cell-color 1,1:red \
  --cell-bg 2,2-4,4:yellow \
  --cell-bold 1,1

# Draw attention to a range
tbl matrix.csv --cell-color 1,1-3,3:bright-green --cell-bold 1,1-3,3
```

### Row Headers (Spreadsheet Style)

```bash
# Grade book with student names as row headers
echo -e "Student,Math,Science,English\nAlice,95,88,92\nBob,78,92,85\nCarol,88,95,90" | tbl \
  --row-headers \
  --row-header-bold \
  --header-bg-color blue \
  --row-header-bg-color green \
  --column-align 2-4:right

# Financial report with labeled rows
tbl quarterly.csv \
  --row-headers \
  --corner-text "Q4 2024" \
  --corner-color bright-magenta \
  --row-header-align right
```

### Multi-Level Styling (Priority System)

```bash
# Global zebra + column colors + cell highlights
tbl data.csv \
  --alt-row-bg white,bright-black \
  --column-color 2:green \
  --cell-color 1,1:bright-red \
  --cell-bg 3,3:yellow
```

**Styling Priority** (highest to lowest):

1. Cell-specific (`--cell-*`)
2. Row/Column-specific (`--row-*`, `--column-*`)
3. Row headers (`--row-header-*`)
4. Global defaults (`--align`, `--header-*`)
5. Zebra striping (`--alt-row-bg`)

### Complex Example: Sales Dashboard

```bash
echo -e "Product,Q1,Q2,Q3,Q4,Total\nLaptops,120,135,142,158,555\nMice,380,425,410,445,1660\nKeyboards,210,195,220,235,860" | tbl \
  --row-headers \
  --corner-text "2024" \
  --corner-color bright-cyan \
  --corner-bg-color black \
  --header-bold \
  --header-bg-color blue \
  --row-header-bold \
  --row-header-bg-color green \
  --column-align 2-6:right \
  --column-bold 6 \
  --cell-color 6,1:bright-yellow \
  --cell-color 6,2:bright-yellow \
  --cell-color 6,3:bright-yellow \
  --alt-row-bg white,bright-black \
  --style rounded
```

Output concept (colors shown as text):

```
╭─────────────┬─────┬─────┬─────┬─────┬────────╮
│ [2024]      ┆ Q1  ┆ Q2  ┆ Q3  ┆ Q4  ┆ Total  │  ← Blue bg, bold
╞═════════════╪═════╪═════╪═════╪═════╪════════╡
│ [Laptops]   ┆ 120 ┆ 135 ┆ 142 ┆ 158 ┆ [555]  │  ← Green bg (row header), white bg (zebra)
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ [Mice]      ┆ 380 ┆ 425 ┆ 410 ┆ 445 ┆ [1660] │  ← Bright-black bg (zebra)
├╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌┤
│ [Keyboards] ┆ 210 ┆ 195 ┆ 220 ┆ 235 ┆ [860]  │  ← White bg (zebra)
╰─────────────┴─────┴─────┴─────┴─────┴────────╯
```

[...] = Bold, Total column highlighted in yellow

## Real-World Examples

The `examples/` directory contains sample data files demonstrating various use cases. Try these commands:

### 1. Basic Table Display

Simple data, default styling:

```bash
tbl examples/simple.csv
```

```
┌───────┬─────┬─────────────┐
│ Name  ┆ Age ┆ City        │
╞═══════╪═════╪═════════════╡
│ Alice ┆ 30  ┆ New York    │
├╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Bob   ┆ 25  ┆ Los Angeles │
├╌╌╌╌╌╌╌┼╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌┤
│ Carol ┆ 28  ┆ Chicago     │
└───────┴─────┴─────────────┘
```

### 2. Financial Report

Right-aligned numbers, bold totals, highlighted header:

```bash
tbl examples/sales.csv \
  --row-headers \
  --column-align 2-6:right \
  --row-bold 5 \
  --column-bold 6 \
  --header-bg-color blue \
  --header-color white \
  --style rounded
```

Professional financial formatting with row headers (Product names), right-aligned numbers, bold totals row and column.

### 3. Grade Book with Color Coding

Spreadsheet-style with row headers and color-coded averages:

```bash
tbl examples/grades.csv \
  --row-headers \
  --column-align 2-6:right \
  --header-bold \
  --header-color bright-cyan \
  --row-header-bold \
  --cell-color 6,1:green \
  --cell-color 6,3:green \
  --cell-color 6,2:yellow \
  --cell-color 6,4:green \
  --cell-color 6,5:yellow
```

Grade book layout with student names as row headers, right-aligned scores, and color-coded averages (green for A's, yellow for B's).

### 4. Server Status Dashboard

Status monitoring with color indicators and zebra striping:

```bash
tbl examples/servers.csv \
  --alt-row-bg white,bright-black \
  --row-color 5:yellow \
  --row-color 7:red \
  --column-bold 2 \
  --header-bg-color black \
  --header-color cyan \
  --header-bold
```

Server monitoring dashboard with zebra striping for readability, warning row in yellow, error row in red, bold status column.

### 5. Product Catalog

Italic product names, bold prices:

```bash
tbl examples/products.csv \
  --column-italic 2 \
  --column-bold 4 \
  --column-align 4-5:right \
  --header-underline \
  --header-color bright-green
```

Product catalog with italicized product names, bold prices, right-aligned numbers, underlined headers.

### 6. Markdown Table for Documentation

Export tables for README files:

```bash
tbl examples/pricing.csv --style markdown --align center
```

```
┌────────────────────────────────────────────────────┐
│ Plan   Users  Storage    Support       Price       │
╞════════════════════════════════════════════════════╡
│ Free     1       5GB      Community       $0       │
│ Starter  5       50GB     Email           $9       │
│ Professional 25  500GB    Priority        $49      │
│ Business 100     2TB      24/7 Phone      $199     │
│ Enterprise Unlimited Unlimited Dedicated  $499     │
└────────────────────────────────────────────────────┘
```

Perfect for pasting into markdown documentation.

### 7. ASCII Table for Plain Text

Compatible with plain text environments:

```bash
tbl examples/departments.txt --style ascii --padding 2
```

Note: Auto-detects pipe delimiter (`|`)

```
+----------------+-------------+-----------+------------------+
|  Department    |  Employees  |  Budget   |  Manager         |
+================+=============+===========+==================+
|  Engineering   |  45         |  $2.5M    |  Sarah Chen      |
+----------------+-------------+-----------+------------------+
|  Sales         |  28         |  $1.8M    |  Michael Torres  |
+----------------+-------------+-----------+------------------+
|  Marketing     |  15         |  $1.2M    |  Jennifer Liu    |
+----------------+-------------+-----------+------------------+
|  Support       |  22         |  $900K    |  David Park      |
+----------------+-------------+-----------+------------------+
|  HR            |  8          |  $600K    |  Emily Rodriguez |
+----------------+-------------+-----------+------------------+
```

### 8. Matrix Data Without Headers

Numeric matrices and data grids:

```bash
tbl examples/matrix.csv \
  --no-header \
  --align center \
  --style rounded \
  --column-color 1-4:cyan
```

```
╭──────┬──────┬──────┬───────╮
│ 1.2  ┆ 3.4  ┆ 5.6  ┆ 7.8   │
├╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ 2.1  ┆ 4.3  ┆ 6.5  ┆ 8.7   │
├╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ 3.0  ┆ 5.2  ┆ 7.4  ┆ 9.6   │
├╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌┼╌╌╌╌╌╌╌┤
│ 4.9  ┆ 6.1  ┆ 8.3  ┆ 10.5  │
╰──────┴──────┴──────┴───────╯
```

### 9. Benchmark Comparison

Highlighting improvements and regressions:

```bash
tbl examples/benchmark.csv \
  --row-color 2:yellow \
  --row-color 1,3,4,6,7:green \
  --column-align 2-4:right \
  --column-bold 4 \
  --header-bg-color black \
  --header-color white \
  --alt-row-bg white,bright-black
```

Benchmark results with green for improvements, yellow for regressions, bold change column.

### 10. Compact Display

Minimal borders for dense data:

```bash
tbl examples/employees.csv \
  --no-outer-border \
  --no-column-border \
  --header-bold \
  --header-color bright-cyan \
  --max-width 120
```

Compact formatting with minimal borders, constrained width for terminal display.

### 11. Financial Statement

Professional accounting layout with row headers:

```bash
tbl examples/financial.csv \
  --row-headers \
  --corner-text "FY2024" \
  --corner-color bright-magenta \
  --column-align 2-5:right \
  --row-bold 3,7 \
  --header-bg-color blue \
  --header-color white \
  --row-header-bg-color green \
  --row-header-bold \
  --style rounded
```

Quarterly financial statement with custom corner cell, bold subtotals (Gross Profit, Net Income), color-coded headers.

### 12. Status Report with Emphasis

Using text attributes for emphasis:

```bash
tbl examples/servers.csv \
  --header-bold \
  --header-underline \
  --column-italic 1 \
  --row-crossed-out 7 \
  --row-dim 4 \
  --column-bold 2
```

Server report with italic server names, bold status, crossed-out offline server, dimmed idle server.

### 13. Cell-Level Highlighting

Precise cell formatting for specific data points:

```bash
tbl examples/grades.csv \
  --cell-bg 2,1:green \
  --cell-bg 2,3:green \
  --cell-color 3,2:red \
  --cell-bold 6,1-6,5 \
  --column-align 2-6:right
```

Grade book with green background for high scores, red text for low score, bold average row.

### 14. Dashboard View with Reverse

High-contrast dashboard using reverse attribute:

```bash
tbl examples/benchmark.csv \
  --header-reverse \
  --row-reverse 1,3,4,6,7 \
  --column-align 2-4:right
```

Reverse video highlighting for improved/stable metrics.

### 15. Professional Report

Combining multiple features for polished output:

```bash
tbl examples/sales.csv \
  --style rounded \
  --row-headers \
  --corner-text "Q1-Q4" \
  --header-bold \
  --header-bg-color blue \
  --header-color white \
  --row-header-bold \
  --row-header-bg-color bright-black \
  --column-align 2-6:right \
  --column-bold 6 \
  --row-bold 5 \
  --cell-color 6,5:bright-yellow \
  --alt-row-bg white,bright-black
```

Complete sales report with row headers, rounded corners, zebra striping, bold totals, highlighted grand total cell.

## Conditional Formatting

Automatically style cells or rows based on their values using conditional formatting rules.

### Row-Level Conditional Formatting

Apply formatting to entire rows when a condition matches:

#### Color Coding by Status

```bash
tbl examples/test_results.csv \
  --row-color-if "Status=Pass:green" \
  --row-color-if "Status=Fail:red"
```

Highlights passing tests in green and failing tests in red.

#### Numeric Thresholds

```bash
tbl examples/sales.csv \
  --row-bg-if "Total<100000:red" \
  --row-bg-if "Total>=200000:green" \
  --row-color-if "Total<100000:white"
```

Background colors based on sales totals: red for low performers, green for high performers.

#### Text Attributes

```bash
tbl examples/test_results.csv \
  --row-bold-if "Score>=90" \
  --row-dim-if "Score<50" \
  --row-italic-if "Testcontains:Gateway"
```

### Cell-Level Conditional Formatting

Apply formatting only to cells in specific columns:

#### Highlighting Specific Values

```bash
tbl examples/test_results.csv \
  --cell-color-if "Score<50:red" \
  --cell-color-if "Score>=90:green" \
  --cell-bold-if "Score>=95"
```

Colors and bolds only the Score column cells based on their values.

#### Multiple Conditions

```bash
tbl examples/financial.csv \
  --cell-bg-if "Profit<0:red" \
  --cell-color-if "Profit<0:white" \
  --cell-bg-if "Profit>100000:green" \
  --cell-bold-if "Profit>100000"
```

### Condition Syntax

Conditional formatting uses the syntax: `COLUMN OPERATOR VALUE`

**Column Identifiers:**
- By index: `2` (1-based column number)
- By name: `Status` (exact header match)

**Numeric Operators:**
- `<` - Less than
- `<=` - Less than or equal
- `>` - Greater than
- `>=` - Greater than or equal
- `=` - Equal (numeric or string)
- `!=` - Not equal (numeric or string)

**String Operators:**
- `contains:` - Contains substring
- `!contains:` - Does not contain substring
- `starts:` - Starts with
- `ends:` - Ends with

**Examples:**
```bash
# Numeric comparisons
--row-color-if "3<0:red"              # Column 3 less than 0
--row-bg-if "Price>=100:yellow"       # Price column >= 100

# String matching
--row-color-if "Status=Error:red"     # Status equals "Error"
--cell-bold-if "Namecontains:Admin"   # Name contains "Admin"
--row-italic-if "Typestarts:WARN"     # Type starts with "WARN"
```

### Available Conditional Flags

**Row-level** (affects all cells in matching rows):
- `--row-color-if COL:COND:COLOR` - Text color
- `--row-bg-if COL:COND:COLOR` - Background color
- `--row-bold-if COL:COND` - Bold text
- `--row-underline-if COL:COND` - Underlined text
- `--row-dim-if COL:COND` - Dimmed text
- `--row-italic-if COL:COND` - Italic text

**Cell-level** (affects only cells in the specified column):
- `--cell-color-if COL:COND:COLOR` - Text color
- `--cell-bg-if COL:COND:COLOR` - Background color
- `--cell-bold-if COL:COND` - Bold text
- `--cell-underline-if COL:COND` - Underlined text
- `--cell-dim-if COL:COND` - Dimmed text
- `--cell-italic-if COL:COND` - Italic text

## Common Use Cases

### Command Output Formatting

```bash
# Format ls output
ls -la | awk '{print $1","$3","$5","$9}' | tbl --header

# Format process list
ps aux | head -10 | tr -s ' ' ',' | tbl
```

### Data Pipeline

```bash
# Extract, transform, and display
curl -s api.example.com/data.csv | \
  grep "active" | \
  tbl --style ascii --align center
```

### Quick File Preview

```bash
# Preview large CSV
head -20 large_data.csv | tbl --max-width 120
```

### Documentation Generation

```bash
# Create markdown tables for docs
tbl config.csv --style markdown >> README.md
```

## Auto-Detection

### Delimiter Detection

`tbl` automatically detects the most likely delimiter by analyzing:

- Comma (`,`)
- Tab (`\t`)
- Pipe (`|`)
- Semicolon (`;`)

The tool chooses the delimiter that appears most consistently across rows.

### Header Detection

Headers are auto-detected using heuristics:

- First row has all unique values
- First row is non-numeric while second row contains numbers
- First row differs in pattern from data rows

You can override with `--header` or `--no-header` flags.

## Building from Source

### Requirements

- Rust 1.70+ (edition 2021)
- Cargo

### Build

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with examples
cargo run -- examples/data.csv
```

## Library Usage

`tbl` can also be used as a Rust library:

```rust
use tbl::{read_input, parse_csv, render_table, ParserConfig, TableConfig};

fn main() -> anyhow::Result<()> {
    let input = read_input(Some("data.csv".into()))?;
    let data = parse_csv(&input, &ParserConfig::default())?;
    let output = render_table(&data, &TableConfig::default());
    println!("{}", output);
    Ok(())
}
```

## Dependencies

- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [comfy-table](https://github.com/Nukesor/comfy-table) - Table rendering
- [csv](https://github.com/BurntSushi/rust-csv) - CSV parsing
- [anyhow](https://github.com/dtolnay/anyhow) - Error handling

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues.

### Development

```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- data.csv

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

## License

MIT License - see LICENSE file for details

## Author

Kirby Little

## Roadmap

Future enhancements may include:

- [ ] TSV and JSON input support
- [ ] Per-column configuration
- [ ] Color schemes and themes
- [ ] Export to HTML/LaTeX
- [ ] Streaming mode for large files
- [ ] Configuration file support
- [ ] Column filtering and sorting

## Acknowledgments

- Built with the excellent [comfy-table](https://github.com/Nukesor/comfy-table) library
- Inspired by command-line tools like `column`, `csvlook`, and `miller`
