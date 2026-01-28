# Implementation Plan: JSON Input Parsing

**Branch**: `001-json-input` | **Date**: 2026-01-27 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `specs/001-json-input/spec.md`

## Summary

Add JSON input parsing capability to tbl, enabling users to format JSON data (arrays of objects, single objects, columnar format) into tables. This extends the existing CSV-focused tool to handle API responses, configuration files, and data science outputs without requiring external conversion tools. The implementation will reuse the existing table rendering infrastructure while adding a new JSON parser module that converts three JSON formats into the internal TableData structure.

**Technical Approach**: Extend the existing `input.rs` → `parser.rs` → `table.rs` pipeline with a new `json_parser.rs` module. Add `serde_json` dependency for robust JSON parsing with depth tracking. Auto-detection logic will examine input prefix (`[` or `{`) to choose between CSV and JSON parsers. All existing styling, conditional formatting, and output features will work transparently with JSON-sourced data.

## Technical Context

**Language/Version**: Rust 2021 (edition 2021, already established in Cargo.toml)
**Primary Dependencies**:
- Existing: clap 4.5, comfy-table 7.1, csv 1.3, anyhow 1.0
- New: `serde_json = "1.0"` (JSON parsing with depth tracking via custom deserializer)
**Storage**: N/A (stateless CLI tool, no persistence)
**Testing**: cargo test (unit tests for json_parser.rs, integration tests for CLI JSON workflows)
**Target Platform**: Cross-platform CLI (Linux, macOS, Windows - same as existing tool)
**Project Type**: Single project (CLI tool)
**Performance Goals**: JSON parsing within 10% of CSV performance for equivalent data size (per SC-003)
**Constraints**:
- Memory: <100MB for typical JSON files up to 100MB input size (per SC-006)
- Nesting depth: Maximum 128 levels (security constraint per FR-016)
- No streaming for MVP (defer to future per Out of Scope)
**Scale/Scope**: Single-file JSON documents up to 100MB, three JSON table formats

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### Compliance Status: ✅ PASSED

**I. Unix Philosophy First**
- ✅ stdin/file input (FR-001, FR-002) - same pattern as CSV
- ✅ stdout output only (reuses existing table rendering)
- ✅ stderr for errors (FR-011)
- ✅ Exit code 0 on success (FR-018 for empty JSON)
- ✅ Pipeline composition maintained (`curl | tbl | ...`)
- ✅ No interactive prompts (fully scriptable)
- ✅ No side effects (parser → renderer, no I/O)

**II. Discoverability & Ergonomics**
- ✅ `--json` flag for forced JSON mode (FR-012)
- ✅ Auto-detection makes JSON transparent when input is `.json` file (FR-002)
- ✅ Help text will document `--json` flag with examples
- ✅ Shell completions will include `--json` flag
- ✅ Default behavior sensible (auto-detect format)

**III. Standards Compliance**
- ✅ Scoped flag `--json` (not `--format json`) matches existing `--csv` pattern
- ✅ Multi-value syntax N/A (JSON structure defines table)
- ✅ Shell-safe (no special characters in flag)
- ✅ "Last wins" for conflicting `--json --csv` flags
- ✅ `--version` and `--help` unchanged

**IV. Performance & Efficiency**
- ✅ Rust 2021 maintained
- ✅ Streaming where possible (serde_json streaming parser for array elements)
- ✅ Zero-cost abstractions (serde zero-copy deserialization where applicable)
- ✅ Performance target: within 10% of CSV (SC-003)
- ✅ Memory target: <100MB for 100MB JSON (SC-006)
- ✅ Efficient iteration (reuse existing TableData structure)

**V. User-Facing Clarity**
- ✅ 1-based indexing unchanged (applies to column/row styling on JSON-sourced tables)
- ✅ Error messages clear: "JSON nesting depth exceeds maximum (128 levels)"
- ✅ Natural language: "Invalid JSON at line 5, column 12"
- ✅ Terminology consistent (column headers, rows, cells)

**VI. Clean Architecture**
- ✅ Module structure: `input.rs` → `json_parser.rs` (new) → `table.rs`
- ✅ Public API in `lib.rs` (add `parse_json` function)
- ✅ Config structs immutable (JSONConfig struct for parser options)
- ✅ Result types for all fallible operations
- ✅ Pure functions (parser stateless)
- ✅ Unit tests for json_parser.rs, integration tests for CLI

### Development Standards Compliance

**Documentation Requirements**
- ✅ README update required with JSON examples (per Feature Development Workflow)
- ✅ Examples in `examples/` directory (add sample JSON files)
- ✅ Design decisions documented in this plan
- ✅ Conventional commits enforced

**Testing Discipline**
- ✅ Unit tests for json_parser.rs (all three formats, depth limit, edge cases)
- ✅ Integration tests for CLI (JSON flag, auto-detection, empty input)
- ✅ Edge case coverage per spec (nested structures, inconsistent schemas, empty JSON)
- ✅ Test data in `examples/` (sample JSON files for three formats)

**Simplicity & Maintenance**
- ✅ No premature abstraction (single json_parser.rs module)
- ✅ YAGNI (only three formats specified, no JSONPath/JSONL)
- ✅ Minimal new dependencies (only serde_json added)
- ✅ No feature flags needed (straightforward addition)

### Gate Decision: ✅ PROCEED

No violations. All constitution principles satisfied by design. JSON parser fits cleanly into existing architecture without breaking Unix philosophy, maintains performance standards, and follows established patterns.

## Project Structure

### Documentation (this feature)

```text
specs/001-json-input/
├── plan.md              # This file
├── spec.md              # Feature specification (complete)
├── research.md          # Phase 0: JSON library evaluation
├── data-model.md        # Phase 1: JSON format structures
├── quickstart.md        # Phase 1: Usage examples
└── contracts/           # Phase 1: N/A (no APIs, CLI tool)
```

### Source Code (repository root)

```text
src/
├── error.rs             # Existing: error types
├── input.rs             # Existing: file/stdin reading
├── parser.rs            # Existing: CSV parsing
├── json_parser.rs       # NEW: JSON parsing (three formats)
├── parse_utils.rs       # Existing: CLI argument parsing
├── table.rs             # Existing: table rendering
├── main.rs              # MODIFIED: add --json flag, auto-detection
└── lib.rs               # MODIFIED: export parse_json function

tests/
├── integration/         # MODIFIED: add JSON CLI tests
│   └── json_tests.rs    # NEW: JSON integration tests
└── unit/                # MODIFIED: add json_parser tests
    └── json_parser_tests.rs  # NEW: unit tests for JSON parsing

examples/
├── simple.csv           # Existing
├── sales.csv            # Existing
├── api_response.json    # NEW: array of objects example
├── config.json          # NEW: single object example
└── columnar.json        # NEW: columnar format example
```

**Structure Decision**: Single project structure (Option 1) maintained. This is a CLI tool with no frontend/backend separation. All source in `src/`, tests in `tests/`, examples in `examples/`. The json_parser.rs module integrates into the existing pipeline at the parser layer, following the established `input → parser → table` flow.

## Complexity Tracking

> No constitution violations - this section intentionally empty.

---

## Phase 0: Research Complete ✅

**Output**: [research.md](./research.md)

**Resolved Questions**:
1. JSON parsing library → serde_json 1.0 (de facto standard, depth control, error detail)
2. Depth limit enforcement → Custom deserializer wrapper (128 levels, security)
3. Auto-detection → Peek first char (`[`, `{`) for JSON vs CSV
4. Column ordering → IndexMap for first-occurrence order (FR-017)
5. Empty JSON handling → Return empty TableData (FR-018)
6. Nested stringification → serde_json::to_string (FR-007)
7. Performance strategy → Stream arrays, in-memory for single/columnar
8. Error messages → Wrap serde_json errors with context (FR-011, SC-005)

**New Dependencies**:
- `serde_json = "1.0"` (JSON parsing core)
- `indexmap = "2.0"` (first-occurrence column ordering)

---

## Phase 1: Design Complete ✅

**Outputs**:
- [data-model.md](./data-model.md) - Five new entities, one reused (TableData)
- [quickstart.md](./quickstart.md) - User guide with examples
- contracts/ - N/A (CLI tool, no API contracts)
- CLAUDE.md - Updated agent context with Rust 2021 + JSON tech stack

**Key Design Decisions**:
1. **JSONFormat enum** discriminates three JSON table formats
2. **DepthLimitedDeserializer** wraps serde_json for nesting limit
3. **ColumnIndex** tracks first-occurrence order with IndexMap
4. **FormatDetector** auto-detects JSON vs CSV by first character
5. **Reuse TableData** - no changes to existing rendering infrastructure

**Architecture**:
```
JSON Input → serde_json → JSONFormat → TableData → comfy_table (existing)
                ↑                ↑
        DepthLimitedDeserializer  ColumnIndex
```

---

## Post-Design Constitution Re-Check ✅

**Status**: Still compliant, no new violations introduced

**Key Validations**:
1. **Unix Philosophy**: JSON input follows same stdin/file pattern as CSV
2. **Discoverability**: `--json` flag added, `--help` will document it
3. **Standards Compliance**: Auto-detection is transparent, explicit flag available
4. **Performance**: Streaming for arrays, <10% slower than CSV per research
5. **User Clarity**: Error messages include line/column, natural language
6. **Clean Architecture**: Single new module (json_parser.rs), reuses TableData

**Design Review Against Constitution**:
- ✅ Module structure maintained: `input.rs` → `json_parser.rs` → `table.rs`
- ✅ Pure functions: JSON parser is stateless, no side effects
- ✅ Result types: All fallible operations return `Result<T, Error>`
- ✅ Unit tests planned: json_parser.rs tests for all three formats + edge cases
- ✅ Integration tests planned: CLI workflows (--json flag, auto-detection, errors)
- ✅ No premature abstraction: Single json_parser.rs module, no over-engineering
- ✅ YAGNI: Only three formats implemented, no JSONPath/JSONL/streaming

**Verdict**: Design adheres to all constitution principles. Ready for task generation.

---

## Next Steps

**Ready for**: `/speckit.tasks` command

**What comes next**:
1. Generate tasks.md with implementation checklist
2. Break down work into atomic tasks (parsing, auto-detection, testing, docs)
3. Map tasks to P1/P2/P3 user stories from spec
4. Define dependencies between tasks (e.g., parser → integration tests)

**Estimated Scope** (high-level):
- 1 new module: json_parser.rs (~300-400 lines)
- Modifications: main.rs (add --json flag, auto-detection), lib.rs (export parse_json)
- Tests: ~200 lines unit tests, ~100 lines integration tests
- Examples: 3 JSON sample files
- Docs: README update with JSON examples, CHANGELOG.md entry

**Complexity Assessment**: Medium (clean extension, no breaking changes, reuses infrastructure)

