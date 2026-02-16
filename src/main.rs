use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand, ValueEnum};
use clap_complete::{generate, Shell};
use std::io;
use std::path::PathBuf;

use tbl::{
    parse_csv, parse_utils::{parse_targets, parse_cell_targets, parse_condition, CellTarget, Condition}, read_input, render_table, Alignment, ParserConfig, TableColor, TableConfig,
    TableStyle,
};
use tbl::table::ConditionalFormat;

/// Subcommands for tbl
#[derive(Subcommand, Debug)]
enum Commands {
    /// Generate shell completion scripts
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },
}

/// Format CSV input as pretty tables
#[derive(Parser, Debug)]
#[command(name = "tbl")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

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

    /// Header background color
    #[arg(long, value_enum)]
    header_bg_color: Option<ColorArg>,

    /// Alternating row background colors (zebra striping)
    /// Format: COLOR1,COLOR2 (e.g., white,bright-black)
    #[arg(long, value_name = "COLOR1,COLOR2", value_parser = parse_alt_colors)]
    alt_row_bg: Option<(ColorArg, ColorArg)>,

    // TEXT ATTRIBUTES
    /// Make header text bold
    #[arg(long)]
    header_bold: bool,

    /// Make header text underlined
    #[arg(long)]
    header_underline: bool,

    /// Make header text dim
    #[arg(long)]
    header_dim: bool,

    /// Make header text italic
    #[arg(long)]
    header_italic: bool,

    /// Make header text blink slowly
    #[arg(long)]
    header_slow_blink: bool,

    /// Make header text blink rapidly
    #[arg(long)]
    header_rapid_blink: bool,

    /// Reverse header colors (swap foreground/background)
    #[arg(long)]
    header_reverse: bool,

    /// Make header text hidden
    #[arg(long)]
    header_hidden: bool,

    /// Make header text crossed out (strikethrough)
    #[arg(long)]
    header_crossed_out: bool,

    // COLUMN OPTIONS
    /// Per-column alignment (format: COL:ALIGN or COLS:ALIGN)
    /// Supports ranges (2-4) and lists (2,3,4). Column numbers are 1-based.
    /// Example: --column-align 1:left --column-align 2-4:right
    #[arg(long, value_name = "COLS:ALIGN", value_parser = parse_column_align_range)]
    column_align: Vec<(Vec<usize>, AlignmentArg)>,

    /// Per-column text color (format: COLS:COLOR)
    /// Supports ranges (2-4) and lists (2,3,4).
    /// Example: --column-color 2:red --column-color 3-5:blue
    #[arg(long, value_name = "COLS:COLOR", value_parser = parse_column_color)]
    column_color: Vec<(Vec<usize>, ColorArg)>,

    /// Per-column background color (format: COLS:COLOR)
    /// Example: --column-bg 2:yellow --column-bg 3-5:bright-black
    #[arg(long, value_name = "COLS:COLOR", value_parser = parse_column_bg)]
    column_bg: Vec<(Vec<usize>, ColorArg)>,

    /// Make columns bold (format: COLS)
    /// Supports ranges (2-4) and lists (2,3,4).
    /// Example: --column-bold 1 --column-bold 3-5
    #[arg(long, value_name = "COLS", value_parser = parse_targets_wrapper)]
    column_bold: Vec<Vec<usize>>,

    /// Make columns underlined (format: COLS)
    #[arg(long, value_name = "COLS", value_parser = parse_targets_wrapper)]
    column_underline: Vec<Vec<usize>>,

    /// Make columns dim (format: COLS)
    #[arg(long, value_name = "COLS", value_parser = parse_targets_wrapper)]
    column_dim: Vec<Vec<usize>>,

    /// Make columns italic (format: COLS)
    #[arg(long, value_name = "COLS", value_parser = parse_targets_wrapper)]
    column_italic: Vec<Vec<usize>>,

    /// Make columns blink slowly (format: COLS)
    #[arg(long, value_name = "COLS", value_parser = parse_targets_wrapper)]
    column_slow_blink: Vec<Vec<usize>>,

    /// Make columns blink rapidly (format: COLS)
    #[arg(long, value_name = "COLS", value_parser = parse_targets_wrapper)]
    column_rapid_blink: Vec<Vec<usize>>,

    /// Reverse column colors (format: COLS)
    #[arg(long, value_name = "COLS", value_parser = parse_targets_wrapper)]
    column_reverse: Vec<Vec<usize>>,

    /// Make columns hidden (format: COLS)
    #[arg(long, value_name = "COLS", value_parser = parse_targets_wrapper)]
    column_hidden: Vec<Vec<usize>>,

    /// Make columns crossed out (format: COLS)
    #[arg(long, value_name = "COLS", value_parser = parse_targets_wrapper)]
    column_crossed_out: Vec<Vec<usize>>,

    // ROW OPTIONS
    /// Per-row text color (format: ROWS:COLOR)
    /// Row numbers are 1-based (first data row = 1).
    /// Example: --row-color 1:red --row-color 2-4:green
    #[arg(long, value_name = "ROWS:COLOR", value_parser = parse_row_color)]
    row_color: Vec<(Vec<usize>, ColorArg)>,

    /// Per-row background color (format: ROWS:COLOR)
    /// Example: --row-bg 1:yellow --row-bg 2-4:bright-black
    #[arg(long, value_name = "ROWS:COLOR", value_parser = parse_row_bg)]
    row_bg: Vec<(Vec<usize>, ColorArg)>,

    /// Make rows bold (format: ROWS)
    /// Example: --row-bold 1 --row-bold 3-5
    #[arg(long, value_name = "ROWS", value_parser = parse_targets_wrapper)]
    row_bold: Vec<Vec<usize>>,

    /// Make rows underlined (format: ROWS)
    #[arg(long, value_name = "ROWS", value_parser = parse_targets_wrapper)]
    row_underline: Vec<Vec<usize>>,

    /// Make rows dim (format: ROWS)
    #[arg(long, value_name = "ROWS", value_parser = parse_targets_wrapper)]
    row_dim: Vec<Vec<usize>>,

    /// Make rows italic (format: ROWS)
    #[arg(long, value_name = "ROWS", value_parser = parse_targets_wrapper)]
    row_italic: Vec<Vec<usize>>,

    /// Make rows blink slowly (format: ROWS)
    #[arg(long, value_name = "ROWS", value_parser = parse_targets_wrapper)]
    row_slow_blink: Vec<Vec<usize>>,

    /// Make rows blink rapidly (format: ROWS)
    #[arg(long, value_name = "ROWS", value_parser = parse_targets_wrapper)]
    row_rapid_blink: Vec<Vec<usize>>,

    /// Reverse row colors (format: ROWS)
    #[arg(long, value_name = "ROWS", value_parser = parse_targets_wrapper)]
    row_reverse: Vec<Vec<usize>>,

    /// Make rows hidden (format: ROWS)
    #[arg(long, value_name = "ROWS", value_parser = parse_targets_wrapper)]
    row_hidden: Vec<Vec<usize>>,

    /// Make rows crossed out (format: ROWS)
    #[arg(long, value_name = "ROWS", value_parser = parse_targets_wrapper)]
    row_crossed_out: Vec<Vec<usize>>,

    // CELL OPTIONS
    /// Per-cell text color (format: COORDS:COLOR)
    /// COORDS can be a list (1,1,2,2) or range (1,1-3,3).
    /// Example: --cell-color 1,1:red --cell-color 1,1-3,3:blue
    #[arg(long, value_name = "COORDS:COLOR", value_parser = parse_cell_color)]
    cell_color: Vec<(CellTarget, ColorArg)>,

    /// Per-cell background color (format: COORDS:COLOR)
    /// Example: --cell-bg 1,1:yellow --cell-bg 2,2-4,4:bright-black
    #[arg(long, value_name = "COORDS:COLOR", value_parser = parse_cell_bg)]
    cell_bg: Vec<(CellTarget, ColorArg)>,

    /// Make cells bold (format: COORDS)
    /// Example: --cell-bold 1,1 --cell-bold 2,2-4,4
    #[arg(long, value_name = "COORDS", value_parser = parse_cell_coords)]
    cell_bold: Vec<CellTarget>,

    /// Make cells underlined (format: COORDS)
    #[arg(long, value_name = "COORDS", value_parser = parse_cell_coords)]
    cell_underline: Vec<CellTarget>,

    /// Make cells dim (format: COORDS)
    #[arg(long, value_name = "COORDS", value_parser = parse_cell_coords)]
    cell_dim: Vec<CellTarget>,

    /// Make cells italic (format: COORDS)
    #[arg(long, value_name = "COORDS", value_parser = parse_cell_coords)]
    cell_italic: Vec<CellTarget>,

    /// Make cells blink slowly (format: COORDS)
    #[arg(long, value_name = "COORDS", value_parser = parse_cell_coords)]
    cell_slow_blink: Vec<CellTarget>,

    /// Make cells blink rapidly (format: COORDS)
    #[arg(long, value_name = "COORDS", value_parser = parse_cell_coords)]
    cell_rapid_blink: Vec<CellTarget>,

    /// Reverse cell colors (format: COORDS)
    #[arg(long, value_name = "COORDS", value_parser = parse_cell_coords)]
    cell_reverse: Vec<CellTarget>,

    /// Make cells hidden (format: COORDS)
    #[arg(long, value_name = "COORDS", value_parser = parse_cell_coords)]
    cell_hidden: Vec<CellTarget>,

    /// Make cells crossed out (format: COORDS)
    #[arg(long, value_name = "COORDS", value_parser = parse_cell_coords)]
    cell_crossed_out: Vec<CellTarget>,

    // CONDITIONAL FORMATTING
    /// Color entire row if condition matches (format: COL:CONDITION:COLOR)
    /// Examples: Status=Error:red, 3<0:red, 2>=90:green
    #[arg(long, value_name = "COL:COND:COLOR", value_parser = parse_row_color_if)]
    row_color_if: Vec<(Condition, ColorArg)>,

    /// Background color for row if condition matches
    #[arg(long, value_name = "COL:COND:COLOR", value_parser = parse_row_bg_if)]
    row_bg_if: Vec<(Condition, ColorArg)>,

    /// Make row bold if condition matches
    #[arg(long, value_name = "COL:COND", value_parser = parse_row_bold_if)]
    row_bold_if: Vec<Condition>,

    /// Make row underlined if condition matches
    #[arg(long, value_name = "COL:COND", value_parser = parse_row_underline_if)]
    row_underline_if: Vec<Condition>,

    /// Make row dim if condition matches
    #[arg(long, value_name = "COL:COND", value_parser = parse_row_dim_if)]
    row_dim_if: Vec<Condition>,

    /// Make row italic if condition matches
    #[arg(long, value_name = "COL:COND", value_parser = parse_row_italic_if)]
    row_italic_if: Vec<Condition>,

    /// Color cell if its value matches condition (format: COL:CONDITION:COLOR)
    #[arg(long, value_name = "COL:COND:COLOR", value_parser = parse_cell_color_if)]
    cell_color_if: Vec<(Condition, ColorArg)>,

    /// Background color for cell if value matches
    #[arg(long, value_name = "COL:COND:COLOR", value_parser = parse_cell_bg_if)]
    cell_bg_if: Vec<(Condition, ColorArg)>,

    /// Make cell bold if value matches
    #[arg(long, value_name = "COL:COND", value_parser = parse_cell_bold_if)]
    cell_bold_if: Vec<Condition>,

    /// Make cell underlined if value matches
    #[arg(long, value_name = "COL:COND", value_parser = parse_cell_underline_if)]
    cell_underline_if: Vec<Condition>,

    /// Make cell dim if value matches
    #[arg(long, value_name = "COL:COND", value_parser = parse_cell_dim_if)]
    cell_dim_if: Vec<Condition>,

    /// Make cell italic if value matches
    #[arg(long, value_name = "COL:COND", value_parser = parse_cell_italic_if)]
    cell_italic_if: Vec<Condition>,

    // ROW HEADERS (first column)
    /// Enable row headers (first column contains labels)
    #[arg(long)]
    row_headers: bool,

    /// Row header text color
    #[arg(long, value_enum)]
    row_header_color: Option<ColorArg>,

    /// Row header background color
    #[arg(long, value_enum)]
    row_header_bg_color: Option<ColorArg>,

    /// Make row header text bold
    #[arg(long)]
    row_header_bold: bool,

    /// Make row header text underlined
    #[arg(long)]
    row_header_underline: bool,

    /// Make row header text dim
    #[arg(long)]
    row_header_dim: bool,

    /// Make row header text italic
    #[arg(long)]
    row_header_italic: bool,

    /// Make row header text blink slowly
    #[arg(long)]
    row_header_slow_blink: bool,

    /// Make row header text blink rapidly
    #[arg(long)]
    row_header_rapid_blink: bool,

    /// Reverse row header colors (swap foreground/background)
    #[arg(long)]
    row_header_reverse: bool,

    /// Make row header text hidden
    #[arg(long)]
    row_header_hidden: bool,

    /// Make row header text crossed out (strikethrough)
    #[arg(long)]
    row_header_crossed_out: bool,

    /// Row header alignment
    #[arg(long, value_enum)]
    row_header_align: Option<AlignmentArg>,

    // CORNER CELL (when both column and row headers present)
    /// Custom text for corner cell (top-left)
    #[arg(long, value_name = "TEXT")]
    corner_text: Option<String>,

    /// Corner cell text color
    #[arg(long, value_enum)]
    corner_color: Option<ColorArg>,

    /// Corner cell background color
    #[arg(long, value_enum)]
    corner_bg_color: Option<ColorArg>,

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

/// Parse column alignment with range/multi-target support (format: "COLS:ALIGN")
fn parse_column_align_range(s: &str) -> Result<(Vec<usize>, AlignmentArg), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid format: '{}'. Expected format: COLS:ALIGN (e.g., 1:left or 2-4:right)",
            s
        ));
    }

    let cols = parse_targets(parts[0], "column")?;
    let align = parse_alignment(parts[1])?;

    Ok((cols, align))
}

/// Parse column color (format: "COLS:COLOR")
fn parse_column_color(s: &str) -> Result<(Vec<usize>, ColorArg), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid format: '{}'. Expected format: COLS:COLOR (e.g., 2:red or 2-4:blue)",
            s
        ));
    }

    let cols = parse_targets(parts[0], "column")?;
    let color = parse_color_arg(parts[1])?;

    Ok((cols, color))
}

/// Parse column background color (format: "COLS:COLOR")
fn parse_column_bg(s: &str) -> Result<(Vec<usize>, ColorArg), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid format: '{}'. Expected format: COLS:COLOR (e.g., 2:yellow)",
            s
        ));
    }

    let cols = parse_targets(parts[0], "column")?;
    let color = parse_color_arg(parts[1])?;

    Ok((cols, color))
}

/// Parse row color (format: "ROWS:COLOR")
fn parse_row_color(s: &str) -> Result<(Vec<usize>, ColorArg), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid format: '{}'. Expected format: ROWS:COLOR (e.g., 1:red or 2-4:green)",
            s
        ));
    }

    let rows = parse_targets(parts[0], "row")?;
    let color = parse_color_arg(parts[1])?;

    Ok((rows, color))
}

/// Parse row background color (format: "ROWS:COLOR")
fn parse_row_bg(s: &str) -> Result<(Vec<usize>, ColorArg), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid format: '{}'. Expected format: ROWS:COLOR (e.g., 1:yellow)",
            s
        ));
    }

    let rows = parse_targets(parts[0], "row")?;
    let color = parse_color_arg(parts[1])?;

    Ok((rows, color))
}

/// Parse cell color (format: "COORDS:COLOR")
fn parse_cell_color(s: &str) -> Result<(CellTarget, ColorArg), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid format: '{}'. Expected format: COORDS:COLOR (e.g., 1,1:red or 1,1-3,3:blue)",
            s
        ));
    }

    let coords = parse_cell_targets(parts[0])?;
    let color = parse_color_arg(parts[1])?;

    Ok((coords, color))
}

/// Parse cell background color (format: "COORDS:COLOR")
fn parse_cell_bg(s: &str) -> Result<(CellTarget, ColorArg), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid format: '{}'. Expected format: COORDS:COLOR (e.g., 1,1:yellow)",
            s
        ));
    }

    let coords = parse_cell_targets(parts[0])?;
    let color = parse_color_arg(parts[1])?;

    Ok((coords, color))
}

/// Parse cell coordinates for bold flag (format: "COORDS")
fn parse_cell_coords(s: &str) -> Result<CellTarget, String> {
    parse_cell_targets(s)
}

/// Wrapper for parse_targets to return Vec for clap
fn parse_targets_wrapper(s: &str) -> Result<Vec<usize>, String> {
    parse_targets(s, "target")
}

/// Parse alignment argument
fn parse_alignment(s: &str) -> Result<AlignmentArg, String> {
    match s.to_lowercase().as_str() {
        "left" => Ok(AlignmentArg::Left),
        "center" => Ok(AlignmentArg::Center),
        "right" => Ok(AlignmentArg::Right),
        _ => Err(format!("Invalid alignment: '{}'. Use: left, center, or right", s)),
    }
}

// Conditional formatting parsers

/// Parse row-color-if (format: "COL:CONDITION:COLOR")
fn parse_row_color_if(s: &str) -> Result<(Condition, ColorArg), String> {
    parse_conditional_with_color(s)
}

/// Parse row-bg-if (format: "COL:CONDITION:COLOR")
fn parse_row_bg_if(s: &str) -> Result<(Condition, ColorArg), String> {
    parse_conditional_with_color(s)
}

/// Parse row-bold-if (format: "COL:CONDITION")
fn parse_row_bold_if(s: &str) -> Result<Condition, String> {
    parse_condition(s)
}

/// Parse row-underline-if (format: "COL:CONDITION")
fn parse_row_underline_if(s: &str) -> Result<Condition, String> {
    parse_condition(s)
}

/// Parse row-dim-if (format: "COL:CONDITION")
fn parse_row_dim_if(s: &str) -> Result<Condition, String> {
    parse_condition(s)
}

/// Parse row-italic-if (format: "COL:CONDITION")
fn parse_row_italic_if(s: &str) -> Result<Condition, String> {
    parse_condition(s)
}

/// Parse cell-color-if (format: "COL:CONDITION:COLOR")
fn parse_cell_color_if(s: &str) -> Result<(Condition, ColorArg), String> {
    parse_conditional_with_color(s)
}

/// Parse cell-bg-if (format: "COL:CONDITION:COLOR")
fn parse_cell_bg_if(s: &str) -> Result<(Condition, ColorArg), String> {
    parse_conditional_with_color(s)
}

/// Parse cell-bold-if (format: "COL:CONDITION")
fn parse_cell_bold_if(s: &str) -> Result<Condition, String> {
    parse_condition(s)
}

/// Parse cell-underline-if (format: "COL:CONDITION")
fn parse_cell_underline_if(s: &str) -> Result<Condition, String> {
    parse_condition(s)
}

/// Parse cell-dim-if (format: "COL:CONDITION")
fn parse_cell_dim_if(s: &str) -> Result<Condition, String> {
    parse_condition(s)
}

/// Parse cell-italic-if (format: "COL:CONDITION")
fn parse_cell_italic_if(s: &str) -> Result<Condition, String> {
    parse_condition(s)
}

/// Helper to parse conditional format with color (format: "CONDITION:COLOR")
fn parse_conditional_with_color(s: &str) -> Result<(Condition, ColorArg), String> {
    // Find the last colon to split condition from color
    if let Some(last_colon) = s.rfind(':') {
        let condition_str = &s[..last_colon];
        let color_str = &s[last_colon + 1..];

        let condition = parse_condition(condition_str)?;
        let color = parse_color_arg(color_str)?;

        Ok((condition, color))
    } else {
        Err(format!(
            "Invalid format: '{}'. Expected format: COL:CONDITION:COLOR (e.g., Status=Error:red, 3<0:red)",
            s
        ))
    }
}

/// Parse alternating row colors (format: "color1,color2")
fn parse_alt_colors(s: &str) -> Result<(ColorArg, ColorArg), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 2 {
        return Err(format!(
            "Invalid format: '{}'. Expected format: COLOR1,COLOR2 (e.g., white,bright-black)",
            s
        ));
    }

    let color1 = parse_color_arg(parts[0])?;
    let color2 = parse_color_arg(parts[1])?;

    Ok((color1, color2))
}

/// Parse a single color argument
fn parse_color_arg(s: &str) -> Result<ColorArg, String> {
    match s.to_lowercase().replace('-', "_").as_str() {
        "black" => Ok(ColorArg::Black),
        "red" => Ok(ColorArg::Red),
        "green" => Ok(ColorArg::Green),
        "yellow" => Ok(ColorArg::Yellow),
        "blue" => Ok(ColorArg::Blue),
        "magenta" => Ok(ColorArg::Magenta),
        "cyan" => Ok(ColorArg::Cyan),
        "white" => Ok(ColorArg::White),
        "bright_black" => Ok(ColorArg::BrightBlack),
        "bright_red" => Ok(ColorArg::BrightRed),
        "bright_green" => Ok(ColorArg::BrightGreen),
        "bright_yellow" => Ok(ColorArg::BrightYellow),
        "bright_blue" => Ok(ColorArg::BrightBlue),
        "bright_magenta" => Ok(ColorArg::BrightMagenta),
        "bright_cyan" => Ok(ColorArg::BrightCyan),
        "bright_white" => Ok(ColorArg::BrightWhite),
        _ => Err(format!(
            "Invalid color: '{}'. Use colors like: red, green, bright-red, etc.",
            s
        )),
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Handle subcommands
    if let Some(command) = cli.command {
        match command {
            Commands::Completions { shell } => {
                let mut cmd = Cli::command();
                let name = cmd.get_name().to_string();
                generate(shell, &mut cmd, name, &mut io::stdout());
                return Ok(());
            }
        }
    }

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

    // Build column alignments vector from CLI args
    let mut column_alignments: Vec<Option<Alignment>> = Vec::new();
    for (cols, align) in &cli.column_align {
        for &col_idx in cols {
            // col_idx is already 0-based from parse_targets
            // Ensure vector is large enough
            if column_alignments.len() <= col_idx {
                column_alignments.resize(col_idx + 1, None);
            }
            column_alignments[col_idx] = Some((*align).into());
        }
    }

    // Build column colors vector
    let mut column_colors: Vec<Option<TableColor>> = Vec::new();
    for (cols, color) in &cli.column_color {
        for &col_idx in cols {
            if column_colors.len() <= col_idx {
                column_colors.resize(col_idx + 1, None);
            }
            column_colors[col_idx] = Some((*color).into());
        }
    }

    // Build column background colors vector
    let mut column_bg_colors: Vec<Option<TableColor>> = Vec::new();
    for (cols, color) in &cli.column_bg {
        for &col_idx in cols {
            if column_bg_colors.len() <= col_idx {
                column_bg_colors.resize(col_idx + 1, None);
            }
            column_bg_colors[col_idx] = Some((*color).into());
        }
    }

    // Build column bold vector
    let mut column_bold: Vec<bool> = Vec::new();
    for cols in &cli.column_bold {
        for &col_idx in cols {
            if column_bold.len() <= col_idx {
                column_bold.resize(col_idx + 1, false);
            }
            column_bold[col_idx] = true;
        }
    }

    // Build column underline vector
    let mut column_underline: Vec<bool> = Vec::new();
    for cols in &cli.column_underline {
        for &col_idx in cols {
            if column_underline.len() <= col_idx {
                column_underline.resize(col_idx + 1, false);
            }
            column_underline[col_idx] = true;
        }
    }

    // Build column dim vector
    let mut column_dim: Vec<bool> = Vec::new();
    for cols in &cli.column_dim {
        for &col_idx in cols {
            if column_dim.len() <= col_idx {
                column_dim.resize(col_idx + 1, false);
            }
            column_dim[col_idx] = true;
        }
    }

    // Build column italic vector
    let mut column_italic: Vec<bool> = Vec::new();
    for cols in &cli.column_italic {
        for &col_idx in cols {
            if column_italic.len() <= col_idx {
                column_italic.resize(col_idx + 1, false);
            }
            column_italic[col_idx] = true;
        }
    }

    // Build column slow_blink vector
    let mut column_slow_blink: Vec<bool> = Vec::new();
    for cols in &cli.column_slow_blink {
        for &col_idx in cols {
            if column_slow_blink.len() <= col_idx {
                column_slow_blink.resize(col_idx + 1, false);
            }
            column_slow_blink[col_idx] = true;
        }
    }

    // Build column rapid_blink vector
    let mut column_rapid_blink: Vec<bool> = Vec::new();
    for cols in &cli.column_rapid_blink {
        for &col_idx in cols {
            if column_rapid_blink.len() <= col_idx {
                column_rapid_blink.resize(col_idx + 1, false);
            }
            column_rapid_blink[col_idx] = true;
        }
    }

    // Build column reverse vector
    let mut column_reverse: Vec<bool> = Vec::new();
    for cols in &cli.column_reverse {
        for &col_idx in cols {
            if column_reverse.len() <= col_idx {
                column_reverse.resize(col_idx + 1, false);
            }
            column_reverse[col_idx] = true;
        }
    }

    // Build column hidden vector
    let mut column_hidden: Vec<bool> = Vec::new();
    for cols in &cli.column_hidden {
        for &col_idx in cols {
            if column_hidden.len() <= col_idx {
                column_hidden.resize(col_idx + 1, false);
            }
            column_hidden[col_idx] = true;
        }
    }

    // Build column crossed_out vector
    let mut column_crossed_out: Vec<bool> = Vec::new();
    for cols in &cli.column_crossed_out {
        for &col_idx in cols {
            if column_crossed_out.len() <= col_idx {
                column_crossed_out.resize(col_idx + 1, false);
            }
            column_crossed_out[col_idx] = true;
        }
    }

    // Build row colors vector
    let mut row_colors: Vec<Option<TableColor>> = Vec::new();
    for (rows, color) in &cli.row_color {
        for &row_idx in rows {
            if row_colors.len() <= row_idx {
                row_colors.resize(row_idx + 1, None);
            }
            row_colors[row_idx] = Some((*color).into());
        }
    }

    // Build row background colors vector
    let mut row_bg_colors: Vec<Option<TableColor>> = Vec::new();
    for (rows, color) in &cli.row_bg {
        for &row_idx in rows {
            if row_bg_colors.len() <= row_idx {
                row_bg_colors.resize(row_idx + 1, None);
            }
            row_bg_colors[row_idx] = Some((*color).into());
        }
    }

    // Build row bold vector
    let mut row_bold: Vec<bool> = Vec::new();
    for rows in &cli.row_bold {
        for &row_idx in rows {
            if row_bold.len() <= row_idx {
                row_bold.resize(row_idx + 1, false);
            }
            row_bold[row_idx] = true;
        }
    }

    // Build row underline vector
    let mut row_underline: Vec<bool> = Vec::new();
    for rows in &cli.row_underline {
        for &row_idx in rows {
            if row_underline.len() <= row_idx {
                row_underline.resize(row_idx + 1, false);
            }
            row_underline[row_idx] = true;
        }
    }

    // Build row dim vector
    let mut row_dim: Vec<bool> = Vec::new();
    for rows in &cli.row_dim {
        for &row_idx in rows {
            if row_dim.len() <= row_idx {
                row_dim.resize(row_idx + 1, false);
            }
            row_dim[row_idx] = true;
        }
    }

    // Build row italic vector
    let mut row_italic: Vec<bool> = Vec::new();
    for rows in &cli.row_italic {
        for &row_idx in rows {
            if row_italic.len() <= row_idx {
                row_italic.resize(row_idx + 1, false);
            }
            row_italic[row_idx] = true;
        }
    }

    // Build row slow_blink vector
    let mut row_slow_blink: Vec<bool> = Vec::new();
    for rows in &cli.row_slow_blink {
        for &row_idx in rows {
            if row_slow_blink.len() <= row_idx {
                row_slow_blink.resize(row_idx + 1, false);
            }
            row_slow_blink[row_idx] = true;
        }
    }

    // Build row rapid_blink vector
    let mut row_rapid_blink: Vec<bool> = Vec::new();
    for rows in &cli.row_rapid_blink {
        for &row_idx in rows {
            if row_rapid_blink.len() <= row_idx {
                row_rapid_blink.resize(row_idx + 1, false);
            }
            row_rapid_blink[row_idx] = true;
        }
    }

    // Build row reverse vector
    let mut row_reverse: Vec<bool> = Vec::new();
    for rows in &cli.row_reverse {
        for &row_idx in rows {
            if row_reverse.len() <= row_idx {
                row_reverse.resize(row_idx + 1, false);
            }
            row_reverse[row_idx] = true;
        }
    }

    // Build row hidden vector
    let mut row_hidden: Vec<bool> = Vec::new();
    for rows in &cli.row_hidden {
        for &row_idx in rows {
            if row_hidden.len() <= row_idx {
                row_hidden.resize(row_idx + 1, false);
            }
            row_hidden[row_idx] = true;
        }
    }

    // Build row crossed_out vector
    let mut row_crossed_out: Vec<bool> = Vec::new();
    for rows in &cli.row_crossed_out {
        for &row_idx in rows {
            if row_crossed_out.len() <= row_idx {
                row_crossed_out.resize(row_idx + 1, false);
            }
            row_crossed_out[row_idx] = true;
        }
    }

    // Build cell colors vector
    let mut cell_colors: Vec<((usize, usize), TableColor)> = Vec::new();
    for (cell_target, color) in &cli.cell_color {
        for (col, row) in cell_target.cells() {
            cell_colors.push(((col, row), (*color).into()));
        }
    }

    // Build cell background colors vector
    let mut cell_bg_colors: Vec<((usize, usize), TableColor)> = Vec::new();
    for (cell_target, color) in &cli.cell_bg {
        for (col, row) in cell_target.cells() {
            cell_bg_colors.push(((col, row), (*color).into()));
        }
    }

    // Build cell bold vector
    let mut cell_bold: Vec<(usize, usize)> = Vec::new();
    for cell_target in &cli.cell_bold {
        for (col, row) in cell_target.cells() {
            cell_bold.push((col, row));
        }
    }

    // Build cell underline vector
    let mut cell_underline: Vec<(usize, usize)> = Vec::new();
    for cell_target in &cli.cell_underline {
        for (col, row) in cell_target.cells() {
            cell_underline.push((col, row));
        }
    }

    // Build cell dim vector
    let mut cell_dim: Vec<(usize, usize)> = Vec::new();
    for cell_target in &cli.cell_dim {
        for (col, row) in cell_target.cells() {
            cell_dim.push((col, row));
        }
    }

    // Build cell italic vector
    let mut cell_italic: Vec<(usize, usize)> = Vec::new();
    for cell_target in &cli.cell_italic {
        for (col, row) in cell_target.cells() {
            cell_italic.push((col, row));
        }
    }

    // Build cell slow_blink vector
    let mut cell_slow_blink: Vec<(usize, usize)> = Vec::new();
    for cell_target in &cli.cell_slow_blink {
        for (col, row) in cell_target.cells() {
            cell_slow_blink.push((col, row));
        }
    }

    // Build cell rapid_blink vector
    let mut cell_rapid_blink: Vec<(usize, usize)> = Vec::new();
    for cell_target in &cli.cell_rapid_blink {
        for (col, row) in cell_target.cells() {
            cell_rapid_blink.push((col, row));
        }
    }

    // Build cell reverse vector
    let mut cell_reverse: Vec<(usize, usize)> = Vec::new();
    for cell_target in &cli.cell_reverse {
        for (col, row) in cell_target.cells() {
            cell_reverse.push((col, row));
        }
    }

    // Build cell hidden vector
    let mut cell_hidden: Vec<(usize, usize)> = Vec::new();
    for cell_target in &cli.cell_hidden {
        for (col, row) in cell_target.cells() {
            cell_hidden.push((col, row));
        }
    }

    // Build cell crossed_out vector
    let mut cell_crossed_out: Vec<(usize, usize)> = Vec::new();
    for cell_target in &cli.cell_crossed_out {
        for (col, row) in cell_target.cells() {
            cell_crossed_out.push((col, row));
        }
    }

    // Configure table renderer
    let mut table_config = TableConfig::new();

    // Basic settings
    table_config.style = cli.style.into();
    table_config.max_width = cli.max_width;
    table_config.align = cli.align.into();
    table_config.padding = cli.padding;

    // Borders
    table_config.no_outer_border = cli.no_outer_border;
    table_config.no_column_border = cli.no_column_border;
    table_config.no_header_separator = cli.no_header_separator;

    // Column headers (first row)
    table_config.header_align = cli.header_align.map(|a| a.into());
    table_config.header_color = cli.header_color.map(|c| c.into());
    table_config.header_bg_color = cli.header_bg_color.map(|c| c.into());
    table_config.header_bold = cli.header_bold;
    table_config.header_underline = cli.header_underline;
    table_config.header_dim = cli.header_dim;
    table_config.header_italic = cli.header_italic;
    table_config.header_slow_blink = cli.header_slow_blink;
    table_config.header_rapid_blink = cli.header_rapid_blink;
    table_config.header_reverse = cli.header_reverse;
    table_config.header_hidden = cli.header_hidden;
    table_config.header_crossed_out = cli.header_crossed_out;

    // Row headers (first column)
    table_config.row_headers_enabled = cli.row_headers;
    table_config.row_header_color = cli.row_header_color.map(|c| c.into());
    table_config.row_header_bg_color = cli.row_header_bg_color.map(|c| c.into());
    table_config.row_header_bold = cli.row_header_bold;
    table_config.row_header_underline = cli.row_header_underline;
    table_config.row_header_dim = cli.row_header_dim;
    table_config.row_header_italic = cli.row_header_italic;
    table_config.row_header_slow_blink = cli.row_header_slow_blink;
    table_config.row_header_rapid_blink = cli.row_header_rapid_blink;
    table_config.row_header_reverse = cli.row_header_reverse;
    table_config.row_header_hidden = cli.row_header_hidden;
    table_config.row_header_crossed_out = cli.row_header_crossed_out;
    table_config.row_header_align = cli.row_header_align.map(|a| a.into());

    // Corner cell
    table_config.corner_text = cli.corner_text;
    table_config.corner_color = cli.corner_color.map(|c| c.into());
    table_config.corner_bg_color = cli.corner_bg_color.map(|c| c.into());

    // Column formatting
    table_config.column_alignments = column_alignments;
    table_config.column_colors = column_colors;
    table_config.column_bg_colors = column_bg_colors;
    table_config.column_bold = column_bold;
    table_config.column_underline = column_underline;
    table_config.column_dim = column_dim;
    table_config.column_italic = column_italic;
    table_config.column_slow_blink = column_slow_blink;
    table_config.column_rapid_blink = column_rapid_blink;
    table_config.column_reverse = column_reverse;
    table_config.column_hidden = column_hidden;
    table_config.column_crossed_out = column_crossed_out;

    // Row formatting
    table_config.row_colors = row_colors;
    table_config.row_bg_colors = row_bg_colors;
    table_config.row_bold = row_bold;
    table_config.row_underline = row_underline;
    table_config.row_dim = row_dim;
    table_config.row_italic = row_italic;
    table_config.row_slow_blink = row_slow_blink;
    table_config.row_rapid_blink = row_rapid_blink;
    table_config.row_reverse = row_reverse;
    table_config.row_hidden = row_hidden;
    table_config.row_crossed_out = row_crossed_out;
    table_config.alt_row_bg_colors = cli.alt_row_bg.map(|(c1, c2)| (c1.into(), c2.into()));

    // Cell formatting
    table_config.cell_colors = cell_colors;
    table_config.cell_bg_colors = cell_bg_colors;
    table_config.cell_bold = cell_bold;
    table_config.cell_underline = cell_underline;
    table_config.cell_dim = cell_dim;
    table_config.cell_italic = cell_italic;
    table_config.cell_slow_blink = cell_slow_blink;
    table_config.cell_rapid_blink = cell_rapid_blink;
    table_config.cell_reverse = cell_reverse;
    table_config.cell_hidden = cell_hidden;
    table_config.cell_crossed_out = cell_crossed_out;

    // Build conditional formatting rules
    let mut conditional_row_formats = Vec::new();
    let mut conditional_cell_formats = Vec::new();

    // Row color conditions
    for (condition, color) in &cli.row_color_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.color = Some((*color).into());
        conditional_row_formats.push(rule);
    }

    // Row background conditions
    for (condition, color) in &cli.row_bg_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.bg_color = Some((*color).into());
        conditional_row_formats.push(rule);
    }

    // Row bold conditions
    for condition in &cli.row_bold_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.bold = true;
        conditional_row_formats.push(rule);
    }

    // Row underline conditions
    for condition in &cli.row_underline_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.underline = true;
        conditional_row_formats.push(rule);
    }

    // Row dim conditions
    for condition in &cli.row_dim_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.dim = true;
        conditional_row_formats.push(rule);
    }

    // Row italic conditions
    for condition in &cli.row_italic_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.italic = true;
        conditional_row_formats.push(rule);
    }

    // Cell color conditions
    for (condition, color) in &cli.cell_color_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.color = Some((*color).into());
        conditional_cell_formats.push(rule);
    }

    // Cell background conditions
    for (condition, color) in &cli.cell_bg_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.bg_color = Some((*color).into());
        conditional_cell_formats.push(rule);
    }

    // Cell bold conditions
    for condition in &cli.cell_bold_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.bold = true;
        conditional_cell_formats.push(rule);
    }

    // Cell underline conditions
    for condition in &cli.cell_underline_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.underline = true;
        conditional_cell_formats.push(rule);
    }

    // Cell dim conditions
    for condition in &cli.cell_dim_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.dim = true;
        conditional_cell_formats.push(rule);
    }

    // Cell italic conditions
    for condition in &cli.cell_italic_if {
        let mut rule = ConditionalFormat::new(condition.clone());
        rule.italic = true;
        conditional_cell_formats.push(rule);
    }

    // Add conditional formatting to config
    table_config.conditional_row_formats = conditional_row_formats;
    table_config.conditional_cell_formats = conditional_cell_formats;

    // Render and print table
    let output = render_table(&table_data, &table_config);
    println!("{}", output);

    Ok(())
}
