use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::*;
use comfy_table::*;

use crate::parser::TableData;

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

/// Column alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Alignment {
    Left,
    Center,
    Right,
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
            TableColor::Black => Color::Black,
            TableColor::Red => Color::Red,
            TableColor::Green => Color::Green,
            TableColor::Yellow => Color::Yellow,
            TableColor::Blue => Color::Blue,
            TableColor::Magenta => Color::Magenta,
            TableColor::Cyan => Color::Cyan,
            TableColor::White => Color::White,
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
#[derive(Debug, Clone)]
pub struct TableConfig {
    /// Table style preset
    pub style: TableStyle,
    /// Maximum table width (None = no limit)
    pub max_width: Option<usize>,
    /// Default column alignment
    pub align: Alignment,
    /// Header alignment (None = use default align)
    pub header_align: Option<Alignment>,
    /// Remove outer borders
    pub no_outer_border: bool,
    /// Remove column separators
    pub no_column_border: bool,
    /// Remove header separator
    pub no_header_separator: bool,
    /// Header text color
    pub header_color: Option<TableColor>,
    /// Horizontal padding (left and right)
    pub padding: usize,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            style: TableStyle::Unicode,
            max_width: None,
            align: Alignment::Left,
            header_align: None,
            no_outer_border: false,
            no_column_border: false,
            no_header_separator: false,
            header_color: None,
            padding: 1,
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

    // Add headers if present
    if let Some(headers) = &data.headers {
        let header_align = config.header_align.unwrap_or(config.align);
        let header_cells: Vec<Cell> = headers
            .iter()
            .map(|h| {
                let mut cell = Cell::new(h).set_alignment(header_align.into());
                if let Some(color) = config.header_color {
                    cell = cell.fg(color.into());
                }
                cell
            })
            .collect();

        table.set_header(header_cells);
    }

    // Add data rows
    for row in &data.rows {
        let cells: Vec<Cell> = row
            .iter()
            .map(|cell| Cell::new(cell).set_alignment(config.align.into()))
            .collect();
        table.add_row(cells);
    }

    table.to_string()
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
