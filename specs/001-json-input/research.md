# Research: JSON Input Parsing

**Date**: 2026-01-27
**Feature**: JSON Input Parsing (001-json-input)
**Purpose**: Resolve technical unknowns and evaluate implementation approaches

---

## 1. JSON Parsing Library Selection

### Decision: serde_json 1.0

**Rationale**:
- **De facto standard** in Rust ecosystem (100M+ downloads, mature and stable)
- **Zero-copy deserialization** where possible (performance requirement SC-003)
- **Streaming support** for large files (helps with 100MB target SC-006)
- **Comprehensive error reporting** with line/column information (FR-011 requirement)
- **Depth tracking capability** via custom deserializer (needed for FR-016: 128 level limit)
- **Active maintenance** and excellent documentation
- **Existing project compatibility** (already using serde-compatible crates)

**Alternatives Considered**:
1. **json crate**: Simpler but lacks streaming, depth control, and error detail
   - ❌ No built-in depth limiting
   - ❌ Poor error messages (no line/column info)
   - ✅ Lighter weight

2. **simd-json**: Faster but less portable and more complex
   - ✅ 2-3x faster parsing
   - ❌ Requires unsafe code, AVX2 CPU features
   - ❌ Not cross-platform (fails performance clause IV: "cross-platform")
   - ❌ Overkill for CLI tool (user waits for table render, not parsing)

3. **Custom parser with nom**: Full control but significant dev cost
   - ✅ Complete control over depth, errors, memory
   - ❌ 10x implementation time vs serde_json
   - ❌ Violates YAGNI and simplicity principles
   - ❌ Higher bug risk (JSON spec edge cases)

**Implementation Notes**:
- Use `serde_json::Deserializer` with custom depth-tracking wrapper
- Streaming mode for array-of-objects (`StreamDeserializer` for large arrays)
- Fallback to in-memory `from_str` for single object and columnar formats
- Leverage existing `anyhow::Error` integration (`serde_json::Error` converts cleanly)

---

## 2. JSON Depth Limit Enforcement (128 Levels)

### Decision: Custom Deserializer Wrapper

**Approach**: Wrap `serde_json::Deserializer` with depth-counting visitor that errors at 128 levels.

**Rationale**:
- **Security requirement** (FR-016): prevents stack overflow from malicious JSON
- **Clean error messages**: "JSON nesting depth exceeds maximum (128 levels)"
- **Minimal performance overhead**: single integer increment/decrement per nesting level
- **Standard pattern**: similar to how serde implements other validation

**Implementation Pattern** (from serde_json documentation + security best practices):
```rust
struct DepthLimitedDeserializer<R> {
    inner: serde_json::Deserializer<R>,
    depth: usize,
    max_depth: usize,
}

impl<R: Read> DepthLimitedDeserializer<R> {
    fn new(reader: R, max_depth: usize) -> Self {
        Self {
            inner: serde_json::Deserializer::from_reader(reader),
            depth: 0,
            max_depth,
        }
    }

    fn check_depth(&mut self) -> Result<()> {
        if self.depth > self.max_depth {
            Err(anyhow!("JSON nesting depth exceeds maximum ({} levels)", self.max_depth))
        } else {
            Ok(())
        }
    }
}
```

**Alternatives Considered**:
1. **Post-parse depth validation**: Walk tree after parsing
   - ❌ Too late (already consumed memory/stack for deep nesting)
   - ❌ Doesn't prevent DoS

2. **Recursion limit in deserializer**: Configure serde_json recursion limit
   - ❌ serde_json doesn't expose this configuration
   - ❌ Would require forking library

**Best Practices Reference**: OWASP JSON Security Cheatsheet recommends depth limits of 64-128 for untrusted input.

---

## 3. Auto-Detection Strategy (JSON vs CSV)

### Decision: Peek-based format detection with heuristics

**Approach**:
1. Read first non-whitespace character from input
2. If `[` or `{` → attempt JSON parse, fallback to CSV on error
3. Otherwise → CSV parse
4. Explicit `--json` or `--csv` flags override detection

**Rationale**:
- **User-friendly**: "just works" for `.json` files and piped JSON (SC-001 one-command workflow)
- **Safe fallback**: Invalid JSON gracefully degrades to CSV (won't break existing workflows)
- **Fast**: O(1) decision (single character peek)
- **Unambiguous**: Valid CSV rarely starts with `[` or `{` (edge case acceptable)

**Edge Cases Handled**:
- **Whitespace prefix**: `trim_start()` before checking
- **Comments**: Not standard JSON (return error), CSV parser handles `#` prefix
- **Empty input**: Neither JSON nor CSV, exit 0 per FR-018

**Implementation Pattern**:
```rust
fn detect_format(input: &str) -> Format {
    match input.trim_start().chars().next() {
        Some('[') | Some('{') => Format::Json,
        _ => Format::Csv,
    }
}
```

**Alternatives Considered**:
1. **File extension only**: Require `.json` suffix
   - ❌ Breaks piped input from `curl | tbl`
   - ❌ User friction (must rename files)

2. **Full parse attempt for both formats**: Try JSON first, fallback to CSV
   - ❌ 2x parsing overhead on CSV files
   - ❌ Confusing error messages (JSON parse error shown for CSV)

---

## 4. Column Ordering for Inconsistent Schemas

### Decision: LinkedHashMap for First-Occurrence Order

**Approach**: Use `IndexMap` (insertion-order-preserving HashMap) to track columns as they're encountered.

**Rationale**:
- **Requirement FR-017**: Columns ordered by first occurrence
- **Efficient**: O(1) insert and lookup, O(n) iteration in insertion order
- **Simple**: Standard Rust crate (`indexmap`), no custom data structure needed
- **Predictable**: Matches pandas, jq, and other data tools behavior

**Implementation Pattern**:
```rust
use indexmap::IndexMap;

fn collect_column_headers(objects: &[Value]) -> Vec<String> {
    let mut headers = IndexMap::new();
    for obj in objects {
        if let Value::Object(map) = obj {
            for key in map.keys() {
                headers.entry(key.clone()).or_insert(());  // Preserve first occurrence
            }
        }
    }
    headers.into_iter().map(|(k, _)| k).collect()
}
```

**Alternatives Considered**:
1. **BTreeMap** (alphabetical order):
   - ❌ Violates FR-017 (first occurrence order)

2. **Custom struct with Vec + HashSet**:
   - ❌ Reinvents `IndexMap`
   - ❌ More complex, potential bugs

**Dependency**: Add `indexmap = "2.0"` to Cargo.toml

---

## 5. Empty JSON Handling ([] and {})

### Decision: Early return with empty TableData

**Approach**: Detect empty JSON after parse, return `TableData { headers: vec![], rows: vec![] }`

**Rationale**:
- **Requirement FR-018**: Empty JSON produces no output with exit 0
- **Consistent**: Matches empty CSV behavior (no headers, no rows)
- **Clean**: Table renderer already handles empty TableData gracefully
- **Fast**: O(1) check after parse

**Implementation**:
```rust
fn parse_json_to_table(value: Value) -> Result<TableData> {
    match value {
        Value::Array(arr) if arr.is_empty() => {
            Ok(TableData { headers: vec![], rows: vec![] })
        }
        Value::Object(obj) if obj.is_empty() => {
            Ok(TableData { headers: vec![], rows: vec![] })
        }
        // ... normal parsing
    }
}
```

---

## 6. Nested Structure Stringification

### Decision: serde_json::to_string for nested values

**Approach**: When encountering nested object/array in table cell, serialize back to compact JSON string.

**Rationale**:
- **Requirement FR-007**: Nested structures as inline JSON strings
- **Round-trip safety**: Preserves exact structure (user can copy/paste for deeper inspection)
- **Simple**: Built-in serde_json function, no custom serialization
- **Compact**: Uses minimal whitespace (e.g., `{"a":1,"b":2}` not pretty-printed)

**Implementation**:
```rust
fn cell_value_to_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),  // Empty cell per FR-008
        Value::Array(_) | Value::Object(_) => {
            serde_json::to_string(value).unwrap_or_else(|_| "{}".to_string())
        }
    }
}
```

**Alternatives Considered**:
1. **Custom formatting** (e.g., `[arr[0], arr[1], ...]`):
   - ❌ Loses fidelity (user can't round-trip)
   - ❌ More complex code

2. **Flatten nested objects** into multiple columns:
   - ❌ Out of scope per spec
   - ❌ Ambiguous naming (`obj.field` vs `obj_field`)

---

## 7. Performance Optimization Strategy

### Decision: Streaming for array-of-objects, in-memory for single/columnar

**Approach**:
- **Array of objects**: Use `StreamDeserializer` to process one object at a time
- **Single object & columnar**: Parse entirely into memory (typically small)

**Rationale**:
- **SC-003 target**: Within 10% of CSV performance
- **Memory efficiency**: Large arrays don't load all at once (SC-006: 100MB limit)
- **Simplicity**: Streaming only where beneficial (array format is most common for large data)

**Benchmarking Plan** (defer to implementation):
- Test with 1MB, 10MB, 100MB JSON files (array of 10k, 100k, 1M objects)
- Compare peak memory usage (RSS) against CSV equivalent
- Measure parse time vs csv crate on equivalent data
- Target: <10% slower than CSV, <100MB memory for 100MB input

---

## 8. Error Message Quality

### Decision: Preserve serde_json error context + friendly wrapper

**Approach**: Wrap serde_json errors with context about what was expected.

**Example**:
```rust
serde_json::from_str(input)
    .context("Failed to parse JSON input")
    .context(format!("Expected JSON array of objects, single object, or columnar format"))
```

**Output**:
```
Error: Failed to parse JSON input
Caused by:
    Expected JSON array of objects, single object, or columnar format
Caused by:
    Expected `,` or `}` at line 5 column 23
```

**Rationale**:
- **FR-011**: Clear error messages
- **SC-005**: Include line/column from serde_json
- **User-friendly**: Explains what formats are valid
- **Actionable**: Line/column lets user find exact error location

---

## Summary of Decisions

| Unknown | Decision | Rationale |
|---------|----------|-----------|
| JSON library | serde_json 1.0 | De facto standard, depth control, error detail |
| Depth limiting | Custom deserializer wrapper | Security (FR-016), clean errors, minimal overhead |
| Auto-detection | Peek first char (`[`, `{`) | Fast, safe fallback, user-friendly |
| Column ordering | IndexMap (first-occurrence) | FR-017 requirement, efficient, predictable |
| Empty JSON | Return empty TableData | FR-018, consistent with CSV, clean |
| Nested stringification | serde_json::to_string | FR-007, round-trip safe, simple |
| Performance | Stream arrays, in-memory else | SC-003/SC-006 targets, pragmatic |
| Error messages | Wrap serde_json errors | FR-011/SC-005, actionable, clear |

**New Dependencies**:
- `serde_json = "1.0"` (JSON parsing)
- `indexmap = "2.0"` (first-occurrence column ordering)

**No Unknowns Remaining**: All NEEDS CLARIFICATION items from Technical Context resolved.

