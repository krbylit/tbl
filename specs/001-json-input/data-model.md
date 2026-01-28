# Data Model: JSON Input Parsing

**Date**: 2026-01-27
**Feature**: JSON Input Parsing (001-json-input)
**Purpose**: Define data structures for JSON parsing and format detection

---

## Overview

The JSON parser extends the existing tbl architecture by adding JSON-to-TableData conversion. The key insight is that all three JSON formats (array of objects, single object, columnar) map to the same `TableData` structure already used by the CSV parser, enabling complete reuse of the table rendering infrastructure.

**Data Flow**:
```
JSON Input → serde_json → JSONFormat (enum) → TableData → comfy_table (existing)
```

---

## Core Entities

### 1. JSONFormat (New)

**Purpose**: Discriminate between three JSON table formats after parsing

**Structure**:
```rust
pub enum JSONFormat {
    /// Array of objects: [{"name": "Alice", "age": 30}, ...]
    /// Most common format for API responses and exports
    ArrayOfObjects(Vec<serde_json::Map<String, Value>>),

    /// Single object: {"name": "Alice", "age": 30}
    /// Used for config files or single-record inspection
    SingleObject(serde_json::Map<String, Value>),

    /// Columnar format: {"name": ["Alice", "Bob"], "age": [30, 25]}
    /// Data science tools (pandas, numpy) output format
    ColumnarObject(serde_json::Map<String, Value>),
}
```

**Validation Rules**:
- **ArrayOfObjects**: All elements MUST be objects (not mixed types)
- **ColumnarObject**: All values MUST be arrays (validated before classification)
- **SingleObject**: Default for non-array top-level objects

**State Transitions**:
1. Parse raw JSON string → `serde_json::Value`
2. Classify `Value` → `JSONFormat` variant
3. Convert `JSONFormat` → `TableData`

---

### 2. TableData (Existing - No Changes)

**Purpose**: Unified representation of tabular data from any source (CSV or JSON)

**Structure** (from existing codebase):
```rust
pub struct TableData {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
```

**Fields**:
- **headers**: Column names (from JSON object keys)
- **rows**: Data rows (each row is vector of cell strings)

**JSON Mapping**:
| JSON Format | headers | rows |
|-------------|---------|------|
| Array of objects | Object keys (first-occurrence order) | Object values per row |
| Single object | Object keys (insertion order) | Single row with values |
| Columnar | Object keys (insertion order) | Transposed array elements |

**Invariants** (maintained by JSON parser):
- All rows have same length as headers (empty string for missing values)
- No null values in headers (keys are always strings)
- Empty JSON produces `TableData { headers: vec![], rows: vec![] }`

---

### 3. DepthLimitedDeserializer (New)

**Purpose**: Enforce 128-level nesting depth limit during deserialization (FR-016)

**Structure**:
```rust
pub struct DepthLimitedDeserializer<R> {
    inner: serde_json::Deserializer<serde_json::de::IoRead<R>>,
    current_depth: usize,
    max_depth: usize,
}
```

**Fields**:
- **inner**: Wrapped serde_json deserializer
- **current_depth**: Track nesting level (increment on `{`/`[`, decrement on `}`/`]`)
- **max_depth**: 128 (constant per FR-016 clarification)

**Validation Rules**:
- Error when `current_depth > max_depth` before processing nested structure
- Error message: "JSON nesting depth exceeds maximum (128 levels)"

---

### 4. ColumnIndex (New)

**Purpose**: Track first-occurrence order of columns in array-of-objects with inconsistent schemas (FR-017)

**Structure**:
```rust
use indexmap::IndexMap;

pub struct ColumnIndex {
    map: IndexMap<String, usize>,
}
```

**Fields**:
- **map**: Insertion-order-preserving HashMap (key = column name, value = index)

**Operations**:
- `insert(key: String)`: Add column if not present, return index
- `keys() -> Vec<String>`: Return column names in first-occurrence order
- `index_of(key: &str) -> Option<usize>`: Get column index

**Validation Rules**:
- Keys are case-sensitive (JSON keys are case-sensitive)
- Empty keys allowed (JSON allows `""` as object key)
- Duplicate inserts are no-ops (first occurrence wins)

---

### 5. FormatDetector (New)

**Purpose**: Auto-detect input format (JSON vs CSV) per FR-012

**Structure**:
```rust
pub struct FormatDetector;

impl FormatDetector {
    pub fn detect(input: &str) -> InputFormat {
        match input.trim_start().chars().next() {
            Some('[') | Some('{') => InputFormat::Json,
            _ => InputFormat::Csv,
        }
    }
}

pub enum InputFormat {
    Json,
    Csv,
}
```

**Validation Rules**:
- Whitespace-agnostic (trim leading whitespace before check)
- Empty input defaults to CSV (will error gracefully)

---

## Relationships

```
┌─────────────┐
│ JSON String │
└──────┬──────┘
       │ serde_json::from_str
       ▼
┌──────────────┐
│ Value (enum) │
└──────┬───────┘
       │ classify_format
       ▼
┌────────────────┐
│ JSONFormat     │  ┌──────────────────────┐
│  - Array       │  │ DepthLimitedDeserializer │
│  - Single      │  │ (wraps serde_json)      │
│  - Columnar    │  └──────────────────────┘
└────────┬───────┘
         │ convert_to_table
         ▼
┌──────────────┐
│ TableData    │ ←──── (Also produced by CSV parser)
│  - headers   │
│  - rows      │
└──────┬───────┘
       │ (existing rendering pipeline)
       ▼
┌──────────────┐
│ comfy_table  │
└──────────────┘
```

---

## JSON Value Type Mapping

**Purpose**: Define how JSON primitive types map to table cell strings (FR-008, FR-009, FR-010)

| JSON Type | TableData Cell String | Example |
|-----------|----------------------|---------|
| String | Value as-is | `"Alice"` → `"Alice"` |
| Number | Formatted number | `42` → `"42"`, `3.14` → `"3.14"` |
| Boolean | `"true"` or `"false"` | `true` → `"true"` |
| Null | Empty string `""` | `null` → `""` |
| Array (nested) | Compact JSON string | `[1,2,3]` → `"[1,2,3]"` |
| Object (nested) | Compact JSON string | `{"a":1}` → `"{\"a\":1}"` |

**Implementation**:
```rust
fn value_to_cell_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Null => String::new(),
        Value::Array(_) | Value::Object(_) => {
            serde_json::to_string(value).unwrap_or_else(|_| String::new())
        }
    }
}
```

---

## Edge Case Handling

### Empty JSON (FR-018)

| Input | headers | rows | Exit Code |
|-------|---------|------|-----------|
| `[]` | `vec![]` | `vec![]` | 0 |
| `{}` | `vec![]` | `vec![]` | 0 |

### Inconsistent Schemas (FR-017)

**Example**:
```json
[
  {"name": "Alice", "age": 30},
  {"name": "Bob", "city": "NYC"},
  {"age": 25, "name": "Charlie"}
]
```

**Result**:
- **headers**: `["name", "age", "city"]` (first-occurrence order)
- **rows**:
  - Row 1: `["Alice", "30", ""]` (missing city)
  - Row 2: `["Bob", "", "NYC"]` (missing age)
  - Row 3: `["Charlie", "25", ""]` (missing city)

**Algorithm**:
1. Collect all unique keys with `ColumnIndex` (first-occurrence order)
2. For each object, create row by iterating column headers
3. Insert empty string for missing keys

### Depth Limit Exceeded (FR-016)

**Input** (129 levels of nesting):
```json
{"a": {"a": {"a": ... }}}
```

**Result**: Error with message:
```
Error: JSON nesting depth exceeds maximum (128 levels)
```

**Exit Code**: Non-zero (error condition)

---

## Performance Characteristics

| Format | Memory Complexity | Time Complexity | Notes |
|--------|-------------------|-----------------|-------|
| Array of objects (N objects, M keys) | O(N × M) | O(N × M) | Stream processing reduces peak memory |
| Single object (M keys) | O(M) | O(M) | Trivial, always fits in memory |
| Columnar (M columns, N rows) | O(N × M) | O(N × M) | Transpose requires full buffering |

**Streaming Optimization** (array of objects):
- Use `serde_json::StreamDeserializer` to process one object at a time
- Build `ColumnIndex` incrementally during first pass
- Second pass converts objects to rows (no need to keep all objects in memory simultaneously)

---

## Data Model Summary

**New Entities**: 5
- `JSONFormat` (enum): Discriminate JSON table formats
- `DepthLimitedDeserializer` (struct): Enforce nesting depth limit
- `ColumnIndex` (struct): Track first-occurrence column order
- `FormatDetector` (struct): Auto-detect JSON vs CSV
- `InputFormat` (enum): Classification result

**Existing Entities Reused**: 1
- `TableData` (struct): No changes, full reuse

**No Database/External Storage**: All data ephemeral (stdin → memory → stdout)

**Validation**: Input validation at parse time (serde_json errors), format validation after parse (classify), depth validation during deserialize.

