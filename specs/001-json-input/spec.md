# Feature Specification: JSON Input Parsing

**Feature Branch**: `001-json-input`
**Created**: 2026-01-27
**Status**: Draft
**Input**: User description: "Implement the ability to parse input in JSON format"

## Clarifications

### Session 2026-01-27

- Q: Should the parser enforce a maximum nesting depth limit for security/stability (to prevent DoS from deeply nested JSON)? → A: Enforce depth limit of 128 levels (reject deeper JSON with clear error message)
- Q: When objects in an array have different keys, in what order should columns appear in the table? → A: First occurrence order (columns appear as keys are first encountered in array)
- Q: How should the tool behave when the entire input is an empty JSON array `[]` or empty object `{}`? → A: Output empty table with no headers/rows (silent success, exit code 0)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Parse Array of Objects (Priority: P1)

Users need to format JSON arrays of objects (the most common JSON table format) into readable tables. This is the primary use case for JSON table formatting - converting API responses, database exports, and structured data into terminal-friendly tables.

**Why this priority**: This is the fundamental use case that provides immediate value. Most JSON data representing tables comes in this format (e.g., `[{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]`). This alone makes the feature useful.

**Independent Test**: Can be fully tested by piping JSON array data through tbl and verifying table output with correct columns and rows, delivering immediate value for API response formatting.

**Acceptance Scenarios**:

1. **Given** a JSON file containing an array of objects with consistent keys, **When** user runs `tbl data.json`, **Then** the tool displays a formatted table with column headers from object keys and rows from object values
2. **Given** JSON array piped from another command, **When** user runs `curl api.example.com/users | tbl`, **Then** the tool displays formatted table from JSON response
3. **Given** a JSON array with nested objects at the top level, **When** user runs `tbl --json data.json`, **Then** the tool displays a table with nested values represented as inline JSON strings
4. **Given** a JSON array with some objects missing keys, **When** user formats it as a table, **Then** missing values appear as empty cells without error

---

### User Story 2 - Parse Single Object as Row (Priority: P2)

Users want to format a single JSON object as a single-row table, useful for inspecting individual records or configuration files.

**Why this priority**: Common secondary use case for single-record inspection (e.g., `{"user": "alice", "status": "active", "role": "admin"}`). Builds on P1 infrastructure with minimal additional work.

**Independent Test**: Can be fully tested by formatting single JSON objects and verifying single-row output with keys as headers.

**Acceptance Scenarios**:

1. **Given** a JSON file containing a single object, **When** user runs `tbl config.json`, **Then** the tool displays a single-row table with keys as column headers
2. **Given** a command outputting a single JSON object, **When** user pipes it through tbl, **Then** the tool formats it as a one-row table
3. **Given** a single object with array values, **When** user formats it, **Then** array values are represented as inline JSON strings in cells

---

### User Story 3 - Parse Object of Arrays (Columnar Format) (Priority: P3)

Users need to format columnar JSON data where keys are column names and values are arrays of column values (e.g., `{"name": ["Alice", "Bob"], "age": [30, 25]}`). This format is common in data science tools and some APIs.

**Why this priority**: Less common format but useful for data science workflows (pandas, numpy exports). Provides feature completeness for JSON table parsing.

**Independent Test**: Can be fully tested by formatting columnar JSON and verifying proper transposition into row format.

**Acceptance Scenarios**:

1. **Given** a JSON object where all values are arrays of equal length, **When** user runs `tbl --json data.json`, **Then** the tool transposes it into a table with keys as headers and array indices as rows
2. **Given** columnar JSON with mismatched array lengths, **When** user formats it, **Then** the tool pads shorter columns with empty cells to match the longest column
3. **Given** columnar JSON mixed with non-array values, **When** user formats it, **Then** the tool treats non-array values as single-element columns

---

### Edge Cases

- Deeply nested structures (objects within arrays within objects) exceeding 128 levels of nesting are rejected with error message "JSON nesting depth exceeds maximum (128 levels)"
- JSON with inconsistent schemas (different keys per object in an array) is handled by collecting all unique keys as columns in first-occurrence order, inserting empty cells where keys are missing
- JSON null values are displayed as empty cells (consistent with CSV behavior)
- Empty arrays `[]` and empty objects `{}` as nested values within table cells are stringified as `[]` and `{}` in those cells
- Empty top-level JSON input (`[]` for array or `{}` for object) produces no output with exit code 0 (silent success, consistent with empty CSV behavior)
- Very large JSON files that exceed 100MB may fail with error message "File too large (maximum supported size: 100MB)" (documented limitation)
- Invalid JSON (syntax errors, incomplete data) produces clear error message including line/column position when available
- JSON boolean values (`true`/`false`) are displayed as text `true` or `false` in table cells
- JSON keys containing special characters or whitespace are preserved as-is in column headers
- JSON with numeric keys or non-string keys has keys converted to strings for column headers

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST parse valid JSON from stdin when input format is JSON
- **FR-002**: System MUST parse valid JSON from file when file extension is `.json` or explicit JSON flag provided
- **FR-003**: System MUST detect JSON array of objects format and convert to table with object keys as column headers
- **FR-004**: System MUST detect single JSON object format and convert to single-row table with object keys as column headers
- **FR-005**: System MUST detect columnar JSON format (object of arrays) and transpose to row-based table
- **FR-006**: System MUST handle missing keys in array of objects by inserting empty cells
- **FR-007**: System MUST represent nested JSON structures as inline JSON strings in table cells
- **FR-008**: System MUST display JSON null values as empty cells (consistent with CSV behavior)
- **FR-009**: System MUST display JSON boolean values as `true` or `false` text
- **FR-010**: System MUST display JSON number values without quotes (as numeric data)
- **FR-011**: System MUST provide clear error message when input is invalid JSON
- **FR-012**: System MUST provide flag `--json` to force JSON parsing (when auto-detection ambiguous)
- **FR-013**: System MUST preserve existing CSV parsing behavior when JSON format not detected
- **FR-014**: System MUST apply all existing styling options (colors, alignment, borders) to JSON-sourced tables
- **FR-015**: System MUST support conditional formatting on JSON-sourced data (same as CSV)
- **FR-016**: System MUST reject JSON with nesting depth exceeding 128 levels with clear error message indicating maximum depth exceeded
- **FR-017**: System MUST order columns by first occurrence when collecting keys from array of objects with inconsistent schemas
- **FR-018**: System MUST handle empty top-level JSON input (`[]` or `{}`) by producing no output with exit code 0

### Key Entities

- **JSON Array of Objects**: Array where each element is an object with string keys - the primary table format
- **JSON Single Object**: Single object with string keys - becomes a one-row table
- **Columnar JSON**: Object where all values are arrays of equal (or similar) length - requires transposition
- **Nested Structure**: Objects or arrays within table values - represented as inline JSON strings (up to 128 levels deep)
- **Table Cell**: Individual data point from JSON (string, number, boolean, null, or stringified nested structure)

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can format JSON array data into tables without manual conversion (one-command workflow: `curl api | tbl`)
- **SC-002**: System correctly parses and formats all three JSON table formats (array of objects, single object, columnar) without errors
- **SC-003**: JSON parsing performance matches CSV parsing performance (within 10% for equivalent data size)
- **SC-004**: All existing tbl styling features work identically with JSON input (100% feature parity)
- **SC-005**: Error messages for invalid JSON are actionable (include line/column of parse error when available)
- **SC-006**: Users can format JSON files up to 100MB without memory issues or crashes

## Assumptions *(optional but recommended)*

- JSON input is UTF-8 encoded (standard for JSON)
- Auto-detection prioritizes JSON when input starts with `[` or `{` and is valid JSON
- Nested arrays in table cells are stringified as `[1,2,3]` rather than expanded into multiple rows
- Nested objects in table cells are stringified as `{"key":"value"}` rather than flattened into multiple columns
- Empty top-level JSON (`[]` or `{}`) produces no output (matches pandas/jq behavior and is consistent with empty CSV files)
- Column order in output follows JSON key order (insertion order for modern JSON parsers)
- For inconsistent schemas, columns appear in first-occurrence order (preserves data structure intent, matches pandas/jq behavior)
- Implementation uses `indexmap` crate for insertion-order-preserving HashMap to maintain first-occurrence column ordering
- Maximum nesting depth of 128 levels is sufficient for legitimate use cases (pathological nesting is rejected for security/stability)
- Extremely large JSON files may require streaming parser (out of scope for MVP - document as known limitation)

## Out of Scope *(optional but recommended)*

- JSON streaming/parsing for files larger than available memory (defer to future enhancement)
- JSONPath queries or filtering before table formatting (users can use `jq | tbl` pipeline)
- Automatic flattening of nested objects into separate columns (users should pre-process with `jq`)
- JSON Schema validation (invalid structure handled gracefully but not validated against schema)
- Pretty-printing JSON output format (tool outputs tables, not JSON)
- JSONL/NDJSON format (newline-delimited JSON) - defer to future enhancement
