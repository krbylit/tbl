use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::*;
use comfy_table::*;

use crate::parser::TableData;
use crate::parse_utils::Condition;

/// Conditional formatting rule
#[derive(Debug, Clone)]
pub struct ConditionalFormat {
    pub condition: Condition,
    pub color: Option<TableColor>,
    pub bg_color: Option<TableColor>,
    pub bold: bool,
    pub underline: bool,
    pub dim: bool,
    pub italic: bool,
}

impl ConditionalFormat {
    pub fn new(condition: Condition) -> Self {
        Self {
            condition,
            color: None,
            bg_color: None,
            bold: false,
            underline: false,
            dim: false,
            italic: false,
        }
    }

    /// Check if this rule applies to the given row
    pub fn applies_to_row(&self, row: &[String], headers: Option<&[String]>) -> bool {
        // Get the column index
        if let Some(col_idx) = self.condition.get_column_index(headers) {
            // Check if the column exists in this row
            if let Some(cell_value) = row.get(col_idx) {
                return self.condition.matches(cell_value);
            }
        }
        false
    }
}

/// Table style presets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableStyle {
    Ascii,
    Unicode,
    Markdown,
    Rounded,
    Sharp,
    Dots,
}

impl Default for TableStyle {
    fn default() -> Self {
        TableStyle::Unicode
    }
}

/// Column alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Alignment::Left
    }
}

impl From<Alignment> for CellAlignment {
    fn from(align: Alignment) -> Self {
        match align {
            Alignment::Left => CellAlignment::Left,
            Alignment::Center => CellAlignment::Center,
            Alignment::Right => CellAlignment::Right,
        }
    }
}

/// Color options for table elements
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableColor {
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

impl From<TableColor> for Color {
    fn from(color: TableColor) -> Self {
        match color {
            // Normal colors map to Dark* variants (ANSI 30-37)
            TableColor::Black => Color::Black,
            TableColor::Red => Color::DarkRed,
            TableColor::Green => Color::DarkGreen,
            TableColor::Yellow => Color::DarkYellow,
            TableColor::Blue => Color::DarkBlue,
            TableColor::Magenta => Color::DarkMagenta,
            TableColor::Cyan => Color::DarkCyan,
            TableColor::White => Color::Grey,

            // Bright colors map to Light* variants (ANSI 90-97)
            TableColor::BrightBlack => Color::DarkGrey,
            TableColor::BrightRed => Color::Red,
            TableColor::BrightGreen => Color::Green,
            TableColor::BrightYellow => Color::Yellow,
            TableColor::BrightBlue => Color::Blue,
            TableColor::BrightMagenta => Color::Magenta,
            TableColor::BrightCyan => Color::Cyan,
            TableColor::BrightWhite => Color::White,
        }
    }
}

/// Configuration for table rendering
#[derive(Debug, Clone, Default)]
pub struct TableConfig {
    // STYLE
    /// Table style preset
    #[allow(clippy::derivable_impls)]
    pub style: TableStyle,
    /// Maximum table width (None = no limit)
    pub max_width: Option<usize>,
    /// Horizontal padding (left and right)
    pub padding: usize,

    // BORDERS
    /// Remove outer borders
    pub no_outer_border: bool,
    /// Remove column separators
    pub no_column_border: bool,
    /// Remove header separator
    pub no_header_separator: bool,

    // GLOBAL ALIGNMENT
    /// Default column alignment
    pub align: Alignment,

    // COLUMN HEADERS (first row)
    /// Header alignment (None = use default align)
    pub header_align: Option<Alignment>,
    /// Header text color
    pub header_color: Option<TableColor>,
    /// Header background color
    pub header_bg_color: Option<TableColor>,
    /// Make header text bold
    pub header_bold: bool,
    /// Make header text underlined
    pub header_underline: bool,
    /// Make header text dim
    pub header_dim: bool,
    /// Make header text italic
    pub header_italic: bool,
    /// Make header text blink slowly
    pub header_slow_blink: bool,
    /// Make header text blink rapidly
    pub header_rapid_blink: bool,
    /// Reverse header colors (swap fg/bg)
    pub header_reverse: bool,
    /// Make header text hidden
    pub header_hidden: bool,
    /// Make header text crossed out
    pub header_crossed_out: bool,

    // ROW HEADERS (first column)
    /// Enable row headers (first column)
    pub row_headers_enabled: bool,
    /// Row header text color
    pub row_header_color: Option<TableColor>,
    /// Row header background color
    pub row_header_bg_color: Option<TableColor>,
    /// Make row header text bold
    pub row_header_bold: bool,
    /// Make row header text underlined
    pub row_header_underline: bool,
    /// Make row header text dim
    pub row_header_dim: bool,
    /// Make row header text italic
    pub row_header_italic: bool,
    /// Make row header text blink slowly
    pub row_header_slow_blink: bool,
    /// Make row header text blink rapidly
    pub row_header_rapid_blink: bool,
    /// Reverse row header colors (swap fg/bg)
    pub row_header_reverse: bool,
    /// Make row header text hidden
    pub row_header_hidden: bool,
    /// Make row header text crossed out
    pub row_header_crossed_out: bool,
    /// Row header alignment
    pub row_header_align: Option<Alignment>,

    // CORNER CELL (intersection of column and row headers)
    /// Text for corner cell (top-left)
    pub corner_text: Option<String>,
    /// Corner cell text color
    pub corner_color: Option<TableColor>,
    /// Corner cell background color
    pub corner_bg_color: Option<TableColor>,

    // COLUMN FORMATTING (per-column overrides)
    /// Per-column alignments (indexed by column number, 0-based)
    pub column_alignments: Vec<Option<Alignment>>,
    /// Per-column text colors
    pub column_colors: Vec<Option<TableColor>>,
    /// Per-column background colors
    pub column_bg_colors: Vec<Option<TableColor>>,
    /// Per-column bold flags
    pub column_bold: Vec<bool>,
    /// Per-column underline flags
    pub column_underline: Vec<bool>,
    /// Per-column dim flags
    pub column_dim: Vec<bool>,
    /// Per-column italic flags
    pub column_italic: Vec<bool>,
    /// Per-column slow blink flags
    pub column_slow_blink: Vec<bool>,
    /// Per-column rapid blink flags
    pub column_rapid_blink: Vec<bool>,
    /// Per-column reverse flags
    pub column_reverse: Vec<bool>,
    /// Per-column hidden flags
    pub column_hidden: Vec<bool>,
    /// Per-column crossed out flags
    pub column_crossed_out: Vec<bool>,

    // ROW FORMATTING (per-row overrides)
    /// Per-row text colors (indexed by data row number, 0-based)
    pub row_colors: Vec<Option<TableColor>>,
    /// Per-row background colors
    pub row_bg_colors: Vec<Option<TableColor>>,
    /// Per-row bold flags
    pub row_bold: Vec<bool>,
    /// Per-row underline flags
    pub row_underline: Vec<bool>,
    /// Per-row dim flags
    pub row_dim: Vec<bool>,
    /// Per-row italic flags
    pub row_italic: Vec<bool>,
    /// Per-row slow blink flags
    pub row_slow_blink: Vec<bool>,
    /// Per-row rapid blink flags
    pub row_rapid_blink: Vec<bool>,
    /// Per-row reverse flags
    pub row_reverse: Vec<bool>,
    /// Per-row hidden flags
    pub row_hidden: Vec<bool>,
    /// Per-row crossed out flags
    pub row_crossed_out: Vec<bool>,
    /// Alternating row background colors (for zebra striping)
    pub alt_row_bg_colors: Option<(TableColor, TableColor)>,

    // CELL FORMATTING (per-cell overrides) - stored as (col, row) -> attribute
    /// Per-cell text colors
    pub cell_colors: Vec<((usize, usize), TableColor)>,
    /// Per-cell background colors
    pub cell_bg_colors: Vec<((usize, usize), TableColor)>,
    /// Per-cell bold flags
    pub cell_bold: Vec<(usize, usize)>,
    /// Per-cell underline flags
    pub cell_underline: Vec<(usize, usize)>,
    /// Per-cell dim flags
    pub cell_dim: Vec<(usize, usize)>,
    /// Per-cell italic flags
    pub cell_italic: Vec<(usize, usize)>,
    /// Per-cell slow blink flags
    pub cell_slow_blink: Vec<(usize, usize)>,
    /// Per-cell rapid blink flags
    pub cell_rapid_blink: Vec<(usize, usize)>,
    /// Per-cell reverse flags
    pub cell_reverse: Vec<(usize, usize)>,
    /// Per-cell hidden flags
    pub cell_hidden: Vec<(usize, usize)>,
    /// Per-cell crossed out flags
    pub cell_crossed_out: Vec<(usize, usize)>,

    // CONDITIONAL FORMATTING (applied based on cell values)
    /// Row-level conditional formatting rules
    pub conditional_row_formats: Vec<ConditionalFormat>,
    /// Cell-level conditional formatting rules
    pub conditional_cell_formats: Vec<ConditionalFormat>,
}

impl TableConfig {
    pub fn new() -> Self {
        Self {
            style: TableStyle::Unicode,
            align: Alignment::Left,
            padding: 1,
            ..Default::default()
        }
    }
}

/// Render table data as a formatted string
///
/// # Arguments
/// * `data` - Parsed table data
/// * `config` - Table rendering configuration
///
/// # Returns
/// Formatted table as a string
pub fn render_table(data: &TableData, config: &TableConfig) -> String {
    let mut table = Table::new();

    // Apply style preset
    apply_style(&mut table, config.style);

    // Apply configuration
    apply_config(&mut table, config);

    // Add column headers if present
    if let Some(headers) = &data.headers {
        let header_cells = build_header_row(headers, config);
        table.set_header(header_cells);
    }

    // Add data rows
    for (row_idx, row) in data.rows.iter().enumerate() {
        let cells = build_data_row(row, row_idx, config, data.headers.as_deref());
        table.add_row(cells);
    }

    table.to_string()
}

/// Build the header row with all styling
fn build_header_row(headers: &[String], config: &TableConfig) -> Vec<Cell> {
    let header_align = config.header_align.unwrap_or(config.align);

    headers
        .iter()
        .enumerate()
        .map(|(col_idx, header_text)| {
            // Special handling for corner cell (when row headers enabled)
            if col_idx == 0 && config.row_headers_enabled {
                return build_corner_cell(header_text, config);
            }

            // Adjust column index if row headers present (corner is col 0)
            let data_col_idx = if config.row_headers_enabled {
                col_idx.saturating_sub(1)
            } else {
                col_idx
            };

            // Get alignment
            let alignment = config
                .column_alignments
                .get(data_col_idx)
                .and_then(|a| *a)
                .unwrap_or(header_align);

            let mut cell = Cell::new(header_text).set_alignment(alignment.into());

            // Apply column header colors
            if let Some(color) = config.header_color {
                cell = cell.fg(color.into());
            }
            if let Some(bg_color) = config.header_bg_color {
                cell = cell.bg(bg_color.into());
            }

            // Apply column header attributes
            if config.header_bold {
                cell = cell.add_attribute(Attribute::Bold);
            }
            if config.header_underline {
                cell = cell.add_attribute(Attribute::Underlined);
            }
            if config.header_dim {
                cell = cell.add_attribute(Attribute::Dim);
            }
            if config.header_italic {
                cell = cell.add_attribute(Attribute::Italic);
            }
            if config.header_slow_blink {
                cell = cell.add_attribute(Attribute::SlowBlink);
            }
            if config.header_rapid_blink {
                cell = cell.add_attribute(Attribute::RapidBlink);
            }
            if config.header_reverse {
                cell = cell.add_attribute(Attribute::Reverse);
            }
            if config.header_hidden {
                cell = cell.add_attribute(Attribute::Hidden);
            }
            if config.header_crossed_out {
                cell = cell.add_attribute(Attribute::CrossedOut);
            }

            cell
        })
        .collect()
}

/// Build the corner cell (top-left intersection when both headers present)
fn build_corner_cell(default_text: &str, config: &TableConfig) -> Cell {
    let text = config
        .corner_text
        .as_deref()
        .unwrap_or(default_text);

    let mut cell = Cell::new(text);

    // Corner cell gets special styling, or inherits from row header
    if let Some(color) = config.corner_color.or(config.row_header_color) {
        cell = cell.fg(color.into());
    }
    if let Some(bg_color) = config.corner_bg_color.or(config.row_header_bg_color) {
        cell = cell.bg(bg_color.into());
    }

    // Apply attributes (combine from both header types)
    if config.header_bold || config.row_header_bold {
        cell = cell.add_attribute(Attribute::Bold);
    }
    if config.header_underline || config.row_header_underline {
        cell = cell.add_attribute(Attribute::Underlined);
    }
    if config.header_dim || config.row_header_dim {
        cell = cell.add_attribute(Attribute::Dim);
    }
    if config.header_italic || config.row_header_italic {
        cell = cell.add_attribute(Attribute::Italic);
    }
    if config.header_slow_blink || config.row_header_slow_blink {
        cell = cell.add_attribute(Attribute::SlowBlink);
    }
    if config.header_rapid_blink || config.row_header_rapid_blink {
        cell = cell.add_attribute(Attribute::RapidBlink);
    }
    if config.header_reverse || config.row_header_reverse {
        cell = cell.add_attribute(Attribute::Reverse);
    }
    if config.header_hidden || config.row_header_hidden {
        cell = cell.add_attribute(Attribute::Hidden);
    }
    if config.header_crossed_out || config.row_header_crossed_out {
        cell = cell.add_attribute(Attribute::CrossedOut);
    }

    cell
}

/// Build a data row with all styling
fn build_data_row(row: &[String], row_idx: usize, config: &TableConfig, headers: Option<&[String]>) -> Vec<Cell> {
    row.iter()
        .enumerate()
        .map(|(col_idx, cell_content)| {
            build_cell(cell_content, col_idx, row_idx, row, config, headers)
        })
        .collect()
}

/// Build a single cell with all applicable styling
fn build_cell(content: &str, col_idx: usize, row_idx: usize, row: &[String], config: &TableConfig, headers: Option<&[String]>) -> Cell {
    // Determine if this is a row header cell
    let is_row_header = col_idx == 0 && config.row_headers_enabled;

    // Get alignment
    let alignment = if is_row_header {
        config.row_header_align.unwrap_or(Alignment::Left)
    } else {
        config
            .column_alignments
            .get(col_idx)
            .and_then(|a| *a)
            .unwrap_or(config.align)
    };

    let mut cell = Cell::new(content).set_alignment(alignment.into());

    // Priority order: Cell-specific > Row/Column-specific > Conditional > Row header > Global > Zebra

    // 1. Apply zebra striping (lowest priority)
    if !is_row_header {
        if let Some((color1, color2)) = config.alt_row_bg_colors {
            let bg_color = if row_idx % 2 == 0 { color1 } else { color2 };
            cell = cell.bg(bg_color.into());
        }
    }

    // 1.5. Apply conditional formatting based on row values (after zebra, before explicit styling)
    if !is_row_header {
        // Check row-level conditional formats
        for rule in &config.conditional_row_formats {
            if rule.applies_to_row(row, headers) {
                if let Some(color) = rule.color {
                    cell = cell.fg(color.into());
                }
                if let Some(bg_color) = rule.bg_color {
                    cell = cell.bg(bg_color.into());
                }
                if rule.bold {
                    cell = cell.add_attribute(Attribute::Bold);
                }
                if rule.underline {
                    cell = cell.add_attribute(Attribute::Underlined);
                }
                if rule.dim {
                    cell = cell.add_attribute(Attribute::Dim);
                }
                if rule.italic {
                    cell = cell.add_attribute(Attribute::Italic);
                }
            }
        }

        // Check cell-level conditional formats (applies to this specific cell)
        for rule in &config.conditional_cell_formats {
            if let Some(check_col_idx) = rule.condition.get_column_index(headers) {
                if check_col_idx == col_idx && rule.condition.matches(content) {
                    if let Some(color) = rule.color {
                        cell = cell.fg(color.into());
                    }
                    if let Some(bg_color) = rule.bg_color {
                        cell = cell.bg(bg_color.into());
                    }
                    if rule.bold {
                        cell = cell.add_attribute(Attribute::Bold);
                    }
                    if rule.underline {
                        cell = cell.add_attribute(Attribute::Underlined);
                    }
                    if rule.dim {
                        cell = cell.add_attribute(Attribute::Dim);
                    }
                    if rule.italic {
                        cell = cell.add_attribute(Attribute::Italic);
                    }
                }
            }
        }
    }

    // 2. Apply row header styling (if applicable)
    if is_row_header {
        if let Some(color) = config.row_header_color {
            cell = cell.fg(color.into());
        }
        if let Some(bg_color) = config.row_header_bg_color {
            cell = cell.bg(bg_color.into());
        }
        if config.row_header_bold {
            cell = cell.add_attribute(Attribute::Bold);
        }
        if config.row_header_underline {
            cell = cell.add_attribute(Attribute::Underlined);
        }
        if config.row_header_dim {
            cell = cell.add_attribute(Attribute::Dim);
        }
        if config.row_header_italic {
            cell = cell.add_attribute(Attribute::Italic);
        }
        if config.row_header_slow_blink {
            cell = cell.add_attribute(Attribute::SlowBlink);
        }
        if config.row_header_rapid_blink {
            cell = cell.add_attribute(Attribute::RapidBlink);
        }
        if config.row_header_reverse {
            cell = cell.add_attribute(Attribute::Reverse);
        }
        if config.row_header_hidden {
            cell = cell.add_attribute(Attribute::Hidden);
        }
        if config.row_header_crossed_out {
            cell = cell.add_attribute(Attribute::CrossedOut);
        }
    }

    // 3. Apply column-specific styling (overrides row header if both apply)
    if !is_row_header {
        if let Some(Some(color)) = config.column_colors.get(col_idx) {
            cell = cell.fg((*color).into());
        }
        if let Some(Some(bg_color)) = config.column_bg_colors.get(col_idx) {
            cell = cell.bg((*bg_color).into());
        }
        if config.column_bold.get(col_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Bold);
        }
        if config.column_underline.get(col_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Underlined);
        }
        if config.column_dim.get(col_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Dim);
        }
        if config.column_italic.get(col_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Italic);
        }
        if config.column_slow_blink.get(col_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::SlowBlink);
        }
        if config.column_rapid_blink.get(col_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::RapidBlink);
        }
        if config.column_reverse.get(col_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Reverse);
        }
        if config.column_hidden.get(col_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Hidden);
        }
        if config.column_crossed_out.get(col_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::CrossedOut);
        }
    }

    // 4. Apply row-specific styling
    if !is_row_header {
        if let Some(Some(color)) = config.row_colors.get(row_idx) {
            cell = cell.fg((*color).into());
        }
        if let Some(Some(bg_color)) = config.row_bg_colors.get(row_idx) {
            cell = cell.bg((*bg_color).into());
        }
        if config.row_bold.get(row_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Bold);
        }
        if config.row_underline.get(row_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Underlined);
        }
        if config.row_dim.get(row_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Dim);
        }
        if config.row_italic.get(row_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Italic);
        }
        if config.row_slow_blink.get(row_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::SlowBlink);
        }
        if config.row_rapid_blink.get(row_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::RapidBlink);
        }
        if config.row_reverse.get(row_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Reverse);
        }
        if config.row_hidden.get(row_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::Hidden);
        }
        if config.row_crossed_out.get(row_idx).copied().unwrap_or(false) {
            cell = cell.add_attribute(Attribute::CrossedOut);
        }
    }

    // 5. Apply cell-specific styling (highest priority)
    if !is_row_header {
        // Check for cell-specific color
        if let Some((_, color)) = config
            .cell_colors
            .iter()
            .find(|((c, r), _)| *c == col_idx && *r == row_idx)
        {
            cell = cell.fg((*color).into());
        }

        // Check for cell-specific background
        if let Some((_, bg_color)) = config
            .cell_bg_colors
            .iter()
            .find(|((c, r), _)| *c == col_idx && *r == row_idx)
        {
            cell = cell.bg((*bg_color).into());
        }

        // Check for cell-specific bold
        if config
            .cell_bold
            .iter()
            .any(|(c, r)| *c == col_idx && *r == row_idx)
        {
            cell = cell.add_attribute(Attribute::Bold);
        }

        // Check for cell-specific underline
        if config
            .cell_underline
            .iter()
            .any(|(c, r)| *c == col_idx && *r == row_idx)
        {
            cell = cell.add_attribute(Attribute::Underlined);
        }

        // Check for cell-specific dim
        if config
            .cell_dim
            .iter()
            .any(|(c, r)| *c == col_idx && *r == row_idx)
        {
            cell = cell.add_attribute(Attribute::Dim);
        }

        // Check for cell-specific italic
        if config
            .cell_italic
            .iter()
            .any(|(c, r)| *c == col_idx && *r == row_idx)
        {
            cell = cell.add_attribute(Attribute::Italic);
        }

        // Check for cell-specific slow blink
        if config
            .cell_slow_blink
            .iter()
            .any(|(c, r)| *c == col_idx && *r == row_idx)
        {
            cell = cell.add_attribute(Attribute::SlowBlink);
        }

        // Check for cell-specific rapid blink
        if config
            .cell_rapid_blink
            .iter()
            .any(|(c, r)| *c == col_idx && *r == row_idx)
        {
            cell = cell.add_attribute(Attribute::RapidBlink);
        }

        // Check for cell-specific reverse
        if config
            .cell_reverse
            .iter()
            .any(|(c, r)| *c == col_idx && *r == row_idx)
        {
            cell = cell.add_attribute(Attribute::Reverse);
        }

        // Check for cell-specific hidden
        if config
            .cell_hidden
            .iter()
            .any(|(c, r)| *c == col_idx && *r == row_idx)
        {
            cell = cell.add_attribute(Attribute::Hidden);
        }

        // Check for cell-specific crossed out
        if config
            .cell_crossed_out
            .iter()
            .any(|(c, r)| *c == col_idx && *r == row_idx)
        {
            cell = cell.add_attribute(Attribute::CrossedOut);
        }
    }

    cell
}

/// Apply style preset to table
fn apply_style(table: &mut Table, style: TableStyle) {
    match style {
        TableStyle::Ascii => {
            table.load_preset(ASCII_FULL);
        }
        TableStyle::Unicode => {
            table.load_preset(UTF8_FULL);
        }
        TableStyle::Markdown => {
            table.load_preset(UTF8_BORDERS_ONLY);
        }
        TableStyle::Rounded => {
            table.load_preset(UTF8_FULL);
            table.apply_modifier(UTF8_ROUND_CORNERS);
        }
        TableStyle::Sharp => {
            table.load_preset(ASCII_FULL);
        }
        TableStyle::Dots => {
            table.load_preset(ASCII_FULL);
        }
    }
}

/// Apply additional configuration to table
fn apply_config(table: &mut Table, config: &TableConfig) {
    // Set width constraint
    if let Some(max_width) = config.max_width {
        table.set_width(max_width as u16);
    }

    // Set content arrangement
    table.set_content_arrangement(ContentArrangement::Dynamic);

    // Note: Some advanced features like custom borders, padding, and separator removal
    // may require more complex manipulation of the table structure.
    // For now, we use the preset styles which handle most common cases.
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::TableData;

    #[test]
    fn test_render_table_with_headers() {
        let data = TableData {
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
            rows: vec![
                vec!["Alice".to_string(), "30".to_string()],
                vec!["Bob".to_string(), "25".to_string()],
            ],
        };

        let config = TableConfig::default();
        let result = render_table(&data, &config);

        assert!(result.contains("Name"));
        assert!(result.contains("Age"));
        assert!(result.contains("Alice"));
        assert!(result.contains("Bob"));
    }

    #[test]
    fn test_render_table_without_headers() {
        let data = TableData {
            headers: None,
            rows: vec![
                vec!["1".to_string(), "2".to_string()],
                vec!["3".to_string(), "4".to_string()],
            ],
        };

        let config = TableConfig::default();
        let result = render_table(&data, &config);

        assert!(result.contains("1"));
        assert!(result.contains("2"));
        assert!(result.contains("3"));
        assert!(result.contains("4"));
    }

    #[test]
    fn test_render_table_ascii_style() {
        let data = TableData {
            headers: Some(vec!["A".to_string()]),
            rows: vec![vec!["1".to_string()]],
        };

        let mut config = TableConfig::default();
        config.style = TableStyle::Ascii;
        let result = render_table(&data, &config);

        // ASCII style should use simple characters
        assert!(result.contains('+') || result.contains('|'));
    }

    #[test]
    fn test_alignment_conversion() {
        assert_eq!(
            CellAlignment::from(Alignment::Left),
            CellAlignment::Left
        );
        assert_eq!(
            CellAlignment::from(Alignment::Center),
            CellAlignment::Center
        );
        assert_eq!(
            CellAlignment::from(Alignment::Right),
            CellAlignment::Right
        );
    }
}
