# Quickstart: JSON Input Parsing

**Date**: 2026-01-27
**Feature**: JSON Input Parsing (001-json-input)
**Purpose**: Quick-start guide for using JSON input with tbl

---

## Installation

**Prerequisites**: Rust 1.75+ (same as existing project)

```bash
# Build from source (development)
git checkout 001-json-input
cargo build --release

# Install locally
cargo install --path .
```

---

## Basic Usage

### Format JSON from API Response

**Most common use case**: Format JSON array of objects from API

```bash
# Pipe API response directly
curl https://api.example.com/users | tbl

# Or from file
tbl users.json
```

**Example Input** (`users.json`):
```json
[
  {"id": 1, "name": "Alice", "role": "admin"},
  {"id": 2, "name": "Bob", "role": "user"},
  {"id": 3, "name": "Charlie", "role": "user"}
]
```

**Output**:
```
┌────┬─────────┬───────┐
│ id │ name    │ role  │
├────┼─────────┼───────┤
│ 1  │ Alice   │ admin │
│ 2  │ Bob     │ user  │
│ 3  │ Charlie │ user  │
└────┴─────────┴───────┘
```

---

### Format Single JSON Object

**Use case**: Inspect configuration files or single records

```bash
tbl config.json
```

**Example Input** (`config.json`):
```json
{
  "host": "localhost",
  "port": 8080,
  "debug": true
}
```

**Output**:
```
┌───────────┬──────┬───────┐
│ host      │ port │ debug │
├───────────┼──────┼───────┤
│ localhost │ 8080 │ true  │
└───────────┴──────┴───────┘
```

---

### Format Columnar JSON (Data Science)

**Use case**: pandas/numpy DataFrame exports

```bash
tbl data.json
```

**Example Input** (`data.json`):
```json
{
  "name": ["Alice", "Bob", "Charlie"],
  "score": [95, 87, 92],
  "passed": [true, true, true]
}
```

**Output**:
```
┌─────────┬───────┬────────┐
│ name    │ score │ passed │
├─────────┼───────┼────────┤
│ Alice   │ 95    │ true   │
│ Bob     │ 87    │ true   │
│ Charlie │ 92    │ true   │
└─────────┴───────┴────────┘
```

---

## Styling JSON Tables

**All existing tbl styling works identically with JSON input** (FR-014)

### Apply Colors

```bash
# Color entire column
curl api/users | tbl --column-color 3:green

# Conditional coloring (role = admin → red)
tbl users.json --row-color-if "role=admin:red"

# Color passing scores green
tbl scores.json --cell-color-if "score>=90:green"
```

### Change Table Style

```bash
# Markdown tables from JSON
curl api/users | tbl --style markdown

# ASCII borders
tbl data.json --style ascii

# Rounded corners
tbl config.json --style rounded
```

### Alignment

```bash
# Right-align numeric columns
tbl users.json --column-align 1:right

# Center all columns
tbl data.json --align center
```

---

## Advanced Features

### Nested JSON Structures

**Nested objects/arrays are stringified** (FR-007)

**Input**:
```json
[
  {"user": "Alice", "tags": ["admin", "dev"]},
  {"user": "Bob", "tags": ["user"]}
]
```

**Output**:
```
┌───────┬──────────────────┐
│ user  │ tags             │
├───────┼──────────────────┤
│ Alice │ ["admin","dev"]  │
│ Bob   │ ["user"]         │
└───────┴──────────────────┘
```

### Inconsistent Schemas

**Missing keys become empty cells** (FR-006, FR-017)

**Input**:
```json
[
  {"name": "Alice", "age": 30},
  {"name": "Bob", "city": "NYC"},
  {"age": 25, "name": "Charlie"}
]
```

**Output** (columns in first-occurrence order):
```
┌─────────┬─────┬──────┐
│ name    │ age │ city │
├─────────┼─────┼──────┤
│ Alice   │ 30  │      │
│ Bob     │     │ NYC  │
│ Charlie │ 25  │      │
└─────────┴─────┴──────┘
```

### Empty JSON

**Empty JSON produces no output** (FR-018)

```bash
echo '[]' | tbl
# (no output, exit code 0)

echo '{}' | tbl
# (no output, exit code 0)
```

---

## Explicit Format Selection

**Auto-detection works for most cases**, but you can force format:

```bash
# Force JSON parsing (useful if file doesn't start with [ or {)
tbl --json data.txt

# Force CSV parsing (disable auto-detection)
tbl --csv data.json  # Treats JSON file as CSV (will likely fail)
```

---

## Error Handling

### Invalid JSON

```bash
$ echo '{invalid' | tbl
Error: Failed to parse JSON input
Caused by:
    Expected JSON array of objects, single object, or columnar format
Caused by:
    EOF while parsing a string at line 1 column 8
```

### Depth Limit Exceeded

```bash
$ tbl deeply_nested.json
Error: JSON nesting depth exceeds maximum (128 levels)
```

### Large Files

```bash
# Files up to 100MB work without issues (SC-006)
curl https://api.example.com/large-dataset | tbl

# Beyond 100MB may hit memory limits (documented limitation)
$ tbl huge.json  # 200MB file
Error: Memory allocation failed (file too large)
```

---

## Integration Examples

### With jq (JSON Processing)

```bash
# Filter then format
jq '.users[] | select(.active == true)' data.json | tbl

# Transform then format
jq '[.[] | {name, email}]' users.json | tbl
```

### With curl (API Calls)

```bash
# GET request
curl -s https://api.github.com/users/octocat/repos | tbl

# POST request with formatting
curl -X POST -d '{"query": "users"}' api/search | tbl
```

### With Docker (Container Output)

```bash
# Format container stats
docker ps --format '{{json .}}' | jq -s | tbl

# Format inspection output
docker inspect container_id | tbl
```

---

## Performance Tips

### Large Arrays

For **very large JSON arrays** (100k+ objects):

1. Use **streaming** (automatic for array format)
2. Consider **filtering with jq first** to reduce data volume
3. Pipe through `head` if you only need first N rows

```bash
# Process only first 100 objects
jq '.[:100]' large.json | tbl

# Filter large dataset before formatting
jq '[.[] | select(.score > 90)]' scores.json | tbl
```

### Memory Usage

**Rule of thumb**: JSON file memory = ~2x file size
- 50MB JSON → ~100MB peak memory
- 100MB JSON → ~200MB peak memory (at limit per SC-006)

---

## Common Patterns

### Dashboard-Style Output

```bash
# Color-code status
curl api/services | tbl \
  --row-color-if "status=up:green" \
  --row-color-if "status=down:red" \
  --row-color-if "status=warning:yellow"
```

### Sorted Output

```bash
# Sort by column before formatting (jq)
jq 'sort_by(.name)' users.json | tbl
```

### Column Subset

```bash
# Select specific fields (jq)
jq '[.[] | {name, email}]' users.json | tbl
```

### Merge Multiple APIs

```bash
# Combine results from multiple endpoints
(curl api/users; curl api/admins) | jq -s 'add' | tbl
```

---

## Troubleshooting

### "Expected JSON array..." Error

**Problem**: Top-level value is not array, object, or columnar format

**Solution**: Wrap in array or fix JSON structure
```bash
# Wrong: bare value
echo '42' | tbl  # Error

# Right: wrap in object or array
echo '{"value": 42}' | tbl
```

### Columns Out of Order

**Problem**: Expecting alphabetical order but getting first-occurrence order

**Explanation**: By design (FR-017), columns appear in the order keys are first seen

**Solution**: Pre-sort with jq if needed
```bash
jq 'map(to_entries | sort_by(.key) | from_entries)' data.json | tbl
```

### Missing Data Shows as Empty

**Problem**: Missing keys in objects show as blank cells

**Explanation**: By design (FR-006), missing keys become empty cells

**Solution**: Fill defaults with jq before formatting
```bash
jq 'map(.city //= "Unknown")' users.json | tbl
```

---

## Next Steps

- See [spec.md](./spec.md) for complete feature specification
- See [data-model.md](./data-model.md) for JSON format details
- Check main README for all styling options
- Run `tbl --help` for full flag reference

