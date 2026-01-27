use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use tbl::{
    parse_csv, read_input, render_table, Alignment, ParserConfig, TableColor, TableConfig,
    TableStyle,
};

/// Format CSV input as pretty tables
#[derive(Parser, Debug)]
#[command(name = "tbl")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input file (or '-' for stdin, default: stdin if not provided)
    #[arg(value_name = "FILE")]
    input: Option<PathBuf>,

    // INPUT FORMAT OPTIONS
    /// Force CSV format (comma-separated, this is the default)
    #[arg(long)]
    csv: bool,

    /// Custom delimiter character
    #[arg(short, long, value_name = "CHAR")]
    delimiter: Option<char>,

    /// First row contains headers (auto-detected by default)
    #[arg(long)]
    header: bool,

    /// First row does NOT contain headers
    #[arg(long, conflicts_with = "header")]
    no_header: bool,

    /// Don't trim whitespace from cells
    #[arg(long)]
    no_trim: bool,

    // TABLE STYLE OPTIONS
    /// Table style preset
    #[arg(short, long, value_enum, default_value_t = StylePreset::Unicode)]
    style: StylePreset,

    /// Maximum table width in characters
    #[arg(short = 'w', long, value_name = "WIDTH")]
    max_width: Option<usize>,

    // ALIGNMENT OPTIONS
    /// Default column alignment
    #[arg(short, long, value_enum, default_value_t = AlignmentArg::Left)]
    align: AlignmentArg,

    /// Header alignment (default: same as columns)
    #[arg(long, value_enum)]
    header_align: Option<AlignmentArg>,

    // BORDER OPTIONS
    /// No outer borders
    #[arg(long)]
    no_outer_border: bool,

    /// No column separators
    #[arg(long)]
    no_column_border: bool,

    /// No header separator line
    #[arg(long)]
    no_header_separator: bool,

    // COLOR OPTIONS
    /// Header text color
    #[arg(long, value_enum)]
    header_color: Option<ColorArg>,

    // PADDING OPTIONS
    /// Horizontal padding (spaces on left and right of cell content)
    #[arg(long, default_value_t = 1)]
    padding: usize,
}

/// Style preset options
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum StylePreset {
    /// ASCII characters only (+-|)
    Ascii,
    /// Unicode box drawing characters (default)
    Unicode,
    /// Markdown-compatible table format
    Markdown,
    /// Rounded corners with Unicode
    Rounded,
    /// Sharp ASCII style
    Sharp,
    /// Dotted borders
    Dots,
}

impl From<StylePreset> for TableStyle {
    fn from(preset: StylePreset) -> Self {
        match preset {
            StylePreset::Ascii => TableStyle::Ascii,
            StylePreset::Unicode => TableStyle::Unicode,
            StylePreset::Markdown => TableStyle::Markdown,
            StylePreset::Rounded => TableStyle::Rounded,
            StylePreset::Sharp => TableStyle::Sharp,
            StylePreset::Dots => TableStyle::Dots,
        }
    }
}

/// Alignment options
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum AlignmentArg {
    /// Left-align text
    Left,
    /// Center-align text
    Center,
    /// Right-align text
    Right,
}

impl From<AlignmentArg> for Alignment {
    fn from(align: AlignmentArg) -> Self {
        match align {
            AlignmentArg::Left => Alignment::Left,
            AlignmentArg::Center => Alignment::Center,
            AlignmentArg::Right => Alignment::Right,
        }
    }
}

/// Color options
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum ColorArg {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl From<ColorArg> for TableColor {
    fn from(color: ColorArg) -> Self {
        match color {
            ColorArg::Black => TableColor::Black,
            ColorArg::Red => TableColor::Red,
            ColorArg::Green => TableColor::Green,
            ColorArg::Yellow => TableColor::Yellow,
            ColorArg::Blue => TableColor::Blue,
            ColorArg::Magenta => TableColor::Magenta,
            ColorArg::Cyan => TableColor::Cyan,
            ColorArg::White => TableColor::White,
            ColorArg::BrightBlack => TableColor::BrightBlack,
            ColorArg::BrightRed => TableColor::BrightRed,
            ColorArg::BrightGreen => TableColor::BrightGreen,
            ColorArg::BrightYellow => TableColor::BrightYellow,
            ColorArg::BrightBlue => TableColor::BrightBlue,
            ColorArg::BrightMagenta => TableColor::BrightMagenta,
            ColorArg::BrightCyan => TableColor::BrightCyan,
            ColorArg::BrightWhite => TableColor::BrightWhite,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Read input from file or stdin
    let input = read_input(cli.input)?;

    // Configure parser
    let parser_config = ParserConfig {
        delimiter: cli.delimiter,
        has_header: if cli.header {
            Some(true)
        } else if cli.no_header {
            Some(false)
        } else {
            None // Auto-detect
        },
        trim_whitespace: !cli.no_trim,
    };

    // Parse CSV
    let table_data = parse_csv(&input, &parser_config)?;

    // Configure table renderer
    let table_config = TableConfig {
        style: cli.style.into(),
        max_width: cli.max_width,
        align: cli.align.into(),
        header_align: cli.header_align.map(|a| a.into()),
        no_outer_border: cli.no_outer_border,
        no_column_border: cli.no_column_border,
        no_header_separator: cli.no_header_separator,
        header_color: cli.header_color.map(|c| c.into()),
        padding: cli.padding,
    };

    // Render and print table
    let output = render_table(&table_data, &table_config);
    println!("{}", output);

    Ok(())
}
