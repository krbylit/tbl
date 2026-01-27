<!--
Sync Impact Report - Constitution Update
=============================================
Version Change: N/A → 1.0.0
Modified Principles: N/A (initial creation)
Added Sections: All (initial creation)
Removed Sections: None
Templates Requiring Updates:
  ✅ .specify/templates/plan-template.md (Constitution Check section compatible)
  ✅ .specify/templates/spec-template.md (no constitution references)
  ✅ .specify/templates/tasks-template.md (no constitution references)
  ✅ .specify/templates/agent-file-template.md (not reviewed - agent-specific)
  ✅ .specify/templates/checklist-template.md (not reviewed - procedural)
Follow-up TODOs: None
Notes: Initial constitution derived from DESIGN_DECISIONS.md core philosophy,
       project patterns in codebase, and README documentation standards.
-->

# tbl Constitution

## Core Principles

### I. Unix Philosophy First

The tool MUST adhere to Unix philosophy: do one thing well, compose with pipes, work as a building block in larger systems.

**Rules:**
- Accept input from stdin OR file (not both simultaneously)
- Output formatted tables to stdout exclusively
- Write errors to stderr
- Exit with 0 on success, non-zero on failure
- Support pipeline composition: `cmd | tbl | cmd`
- No interactive prompts (fully scriptable)
- No side effects (no file writes, network calls, or state changes)

**Rationale:** CLI tools are infrastructure - predictability and composability enable automation and integration. Interactive or stateful behavior breaks pipelines and scripts.

### II. Discoverability & Ergonomics

Every feature MUST be learnable via `--help`. Common cases MUST be simple; complex cases MUST be possible.

**Rules:**
- All flags documented in `--help` output with clear descriptions
- Flag grouping by category (Input, Style, Alignment, Colors, etc.)
- Short flags (`-s`, `-a`, `-w`) for frequently used options only
- Long flags always available (`--style`, `--align`, `--max-width`)
- Examples in `--help` for complex syntax (ranges, coordinates)
- Shell completions provided for all shells (bash, zsh, fish, powershell, elvish)
- Default values sensible for 80% of use cases (unicode style, auto-detection, left alignment)

**Rationale:** Users should learn features progressively without external documentation. Muscle memory from common patterns (git, docker) reduces cognitive load.

### III. Standards Compliance

Follow established CLI conventions from industry-standard tools (git, docker, kubectl, csvkit).

**Rules:**
- Scoped flags: `--column-align` not `--align column` (like git `--pretty`)
- Multi-value syntax: comma-separated lists `2,3,4` (like cut `-f`)
- Range syntax: hyphen for ranges `2-4` (like seq, cut)
- Shell-safe separators: colon `:` for value separation (no quoting required)
- Conflicts handled by "last wins" (standard CLI behavior)
- Version flag: `--version` outputs version only
- Help flag: `--help` outputs full usage, `-h` outputs brief help

**Rationale:** Consistency with existing tools reduces learning curve. Shell-safe syntax prevents quoting errors in scripts.

### IV. Performance & Efficiency

The tool MUST be fast and memory-efficient. Choose Rust and zero-cost abstractions.

**Rules:**
- Use Rust 2021 edition for performance and safety
- Streaming data processing where possible (no full buffering)
- Zero-cost abstractions: no runtime overhead for convenience features
- Benchmark performance for large files (target: >10k rows/sec)
- Profile memory usage (target: <100MB for typical CSVs)
- No unnecessary allocations in hot paths
- Use Vec reuse, string builder patterns, efficient iteration

**Rationale:** CLI tools should feel instant. Users may pipe megabyte CSV files through tbl in production scripts. Slowness breaks workflows.

### V. User-Facing Clarity

Prioritize non-programmer intuition: 1-based indexing, spreadsheet-style terminology, natural language.

**Rules:**
- 1-based indexing for all user-facing flags (columns, rows, cells)
- 0-based indexing internal only (convert at CLI boundary)
- Terminology from spreadsheets: "column headers" (not "header row"), "row headers" (not "header column")
- Row numbering excludes column header (row 1 = first data row)
- Column numbering excludes row header (column 1 = first data column)
- Clear error messages with context: "Invalid column 0 (columns start at 1)" not "parse error"
- Color names in natural language: `bright-red` not `RED_BRIGHT` or `#FF0000`

**Rationale:** Target audience includes non-programmers (analysts, DevOps, docs writers). Spreadsheet conventions (Excel, Google Sheets) are universal. 1-based indexing matches SQL, awk, csvkit.

### VI. Clean Architecture

Maintain service layer separation, module-based structure, functional style.

**Rules:**
- Module structure: `input.rs` → `parser.rs` → `table.rs` → `main.rs`
- Public API in `lib.rs` for library usage (not just CLI)
- Config structs immutable after construction
- Result types for all fallible operations (no panics in library code)
- Pure functions where possible (no hidden state, no global variables)
- Unit tests for parsing logic, integration tests for end-to-end CLI
- Clear separation: main.rs (CLI parsing) vs lib.rs (core logic)

**Rationale:** Clean architecture enables testing, reusability, and maintainability. Library-first design allows embedding tbl in other Rust programs.

## Development Standards

### Documentation Requirements

**Rules:**
- README MUST include quick start, flag reference, real-world examples
- Every example in README MUST be runnable (use `examples/` directory)
- Code comments for non-obvious logic only (self-documenting code preferred)
- Commit messages MUST follow conventional commits (`feat:`, `fix:`, `docs:`)
- Design decisions documented in `DESIGN_DECISIONS.md` with rationale
- Breaking changes documented in CHANGELOG.md with migration paths

**Rationale:** Documentation is code. Examples should be tested by users and updated when features change.

### Testing Discipline

**Rules:**
- Unit tests for all parsing logic (`parse_utils.rs`, `parser.rs`)
- Integration tests for CLI flag combinations
- Edge case coverage: empty tables, out-of-bounds indices, invalid syntax
- Test failure messages MUST be actionable (what failed, expected vs actual)
- No tests committed in failing state (CI must pass)
- Test data in `examples/` directory (real-world representative)

**Rationale:** Parsing edge cases are error-prone. CLI flag interactions are complex. Tests prevent regressions and document intended behavior.

### Simplicity & Maintenance

**Rules:**
- Avoid premature abstraction: three instances before abstracting
- YAGNI (You Aren't Gonna Need It): implement only requested features
- Delete dead code immediately (no commented-out code in commits)
- Refactor when adding third similar pattern
- Feature flags for experimental features (use `cfg` attributes)
- Dependency updates quarterly (security patches immediately)

**Rationale:** Complexity is the enemy of maintainability. Over-engineering creates technical debt. Simpler code is easier to understand and modify.

## Feature Development Workflow

### New Feature Process

**Rules:**
1. Feature request discussed in issue with use case and examples
2. Design decision documented in `DESIGN_DECISIONS.md` before implementation
3. Update `README.md` examples section with new feature examples
4. Implement feature with tests (unit + integration)
5. Update shell completions if new flags added
6. Update `CHANGELOG.md` with feature description
7. Create PR with conventional commit message

**Rationale:** Design-first approach prevents rework. Documentation-driven development ensures features are usable.

### Breaking Changes

**Rules:**
- Breaking changes MUST bump MAJOR version (semantic versioning)
- Breaking changes REQUIRE migration guide in CHANGELOG.md
- Deprecated features get warning messages for one MINOR version before removal
- Flag renames provide alias for one MINOR version (old flag still works)
- Default behavior changes are breaking changes (even if flags unchanged)

**Rationale:** CLI tools are infrastructure. Breaking user scripts is unacceptable without clear migration path and version signal.

## Governance

**Amendment Process:**
- Constitution changes require documentation in this file with rationale
- Version bumped according to semantic versioning (breaking = MAJOR, new principle = MINOR, clarification = PATCH)
- All templates in `.specify/templates/` reviewed for consistency after amendment
- Constitution supersedes all other practices - conflicts resolved in favor of constitution

**Compliance:**
- All PRs MUST verify compliance with applicable principles
- Design decisions MUST reference constitution principles when making trade-offs
- Violations MUST be justified in `plan.md` Complexity Tracking section

**Version**: 1.0.0 | **Ratified**: 2026-01-27 | **Last Amended**: 2026-01-27
