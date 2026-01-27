# tbl - Pretty Table Formatter for CSV Data

A powerful, scriptable CLI tool for formatting CSV data into beautiful tables. Built with Rust for speed and reliability.

## Features

- 📥 **Flexible Input**: Read from files or stdin
- 🔍 **Auto-Detection**: Automatically detects delimiters and headers
- 🎨 **Multiple Styles**: 6 built-in table styles (ASCII, Unicode, Markdown, and more)
- ⚙️ **Highly Configurable**: Extensive options for alignment, borders, colors, and layout
- 🚀 **Fast & Efficient**: Written in Rust for optimal performance
- 📝 **Script-Friendly**: Designed for use in pipelines and automation

## Installation

### From Source

```bash
# Clone or navigate to the project directory
git clone <repository-url>
cd cli_table_util

# Build and install
cargo install --path .

# Or just build
cargo build --release
# Binary will be at: target/release/tbl
```

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

## Usage

```
tbl [OPTIONS] [FILE]
```

### Arguments

- `[FILE]` - Input CSV file (use `-` for stdin, or omit for stdin by default)

### Input Format Options

| Option | Description |
|--------|-------------|
| `--csv` | Force CSV format (comma-separated, default) |
| `-d, --delimiter <CHAR>` | Use custom delimiter character |
| `--header` | Force first row as headers |
| `--no-header` | Force first row as data (no headers) |
| `--no-trim` | Don't trim whitespace from cells |

> **Note**: By default, `tbl` auto-detects delimiters and headers intelligently.

### Table Style Options

| Option | Description |
|--------|-------------|
| `-s, --style <STYLE>` | Table style preset (default: `unicode`) |

**Available Styles**:

- `ascii` - ASCII characters only (`+-|`)
- `unicode` - Unicode box drawing (default)
- `markdown` - Markdown-compatible tables
- `rounded` - Rounded corners with Unicode
- `sharp` - Sharp ASCII style
- `dots` - Dotted borders

### Layout Options

| Option | Description |
|--------|-------------|
| `-w, --max-width <WIDTH>` | Maximum table width in characters |
| `--padding <NUM>` | Horizontal padding (default: 1) |

### Alignment Options

| Option | Description |
|--------|-------------|
| `-a, --align <ALIGN>` | Column alignment: `left`, `center`, `right` (default: `left`) |
| `--header-align <ALIGN>` | Header alignment (default: same as columns) |

### Border Options

| Option | Description |
|--------|-------------|
| `--no-outer-border` | Remove outer borders |
| `--no-column-border` | Remove column separators |
| `--no-header-separator` | Remove header separator line |

### Color Options

| Option | Description |
|--------|-------------|
| `--header-color <COLOR>` | Set header text color |

**Available Colors**: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`, and `bright-*` variants

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

### With Header Colors

```bash
tbl data.csv --header-color green
```

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
