use anyhow::Result;
use std::process::Command;
use tempfile::NamedTempFile;
use std::io::Write;

/// Helper to run tbl CLI and capture output
fn run_tbl(args: &[&str], stdin_data: Option<&str>) -> Result<String> {
    let mut cmd = Command::new("cargo");
    cmd.arg("run");
    cmd.arg("--quiet");
    cmd.arg("--");
    cmd.args(args);

    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    let mut child = cmd.spawn()?;

    if let Some(data) = stdin_data {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(data.as_bytes())?;
        }
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        anyhow::bail!(
            "Command failed with exit code {:?}\nstderr: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(String::from_utf8(output.stdout)?)
}

#[test]
fn test_json_array_from_file() -> Result<()> {
    let output = run_tbl(&["examples/api_response.json"], None)?;

    // Check that output contains expected data
    assert!(output.contains("Alice Johnson"));
    assert!(output.contains("Bob Smith"));
    assert!(output.contains("Charlie Brown"));
    assert!(output.contains("active"));
    assert!(output.contains("email"));

    Ok(())
}

#[test]
fn test_json_array_from_stdin() -> Result<()> {
    let json_input = r#"[
        {"name": "Alice", "age": 30},
        {"name": "Bob", "age": 25}
    ]"#;

    let output = run_tbl(&[], Some(json_input))?;

    assert!(output.contains("Alice"));
    assert!(output.contains("Bob"));
    assert!(output.contains("30"));
    assert!(output.contains("25"));

    Ok(())
}

#[test]
fn test_json_flag_explicit() -> Result<()> {
    let output = run_tbl(&["--json", "examples/api_response.json"], None)?;

    assert!(output.contains("Alice Johnson"));
    assert!(output.contains("admin"));

    Ok(())
}

#[test]
fn test_auto_detection_json() -> Result<()> {
    // Create temp file with JSON content
    let mut temp_file = NamedTempFile::new()?;
    let json_content = r#"[{"city": "NYC", "temp": 72}, {"city": "LA", "temp": 85}]"#;
    temp_file.write_all(json_content.as_bytes())?;

    let output = run_tbl(&[temp_file.path().to_str().unwrap()], None)?;

    // Should auto-detect as JSON and parse correctly
    assert!(output.contains("NYC"));
    assert!(output.contains("LA"));
    assert!(output.contains("72"));
    assert!(output.contains("85"));

    Ok(())
}

#[test]
fn test_styling_works_with_json() -> Result<()> {
    let json_input = r#"[{"name": "Alice"}, {"name": "Bob"}]"#;

    // Test with markdown style
    let output = run_tbl(&["--style", "markdown"], Some(json_input))?;
    assert!(output.contains("Alice"));
    assert!(output.contains("Bob"));

    // Test with ascii style
    let output = run_tbl(&["--style", "ascii"], Some(json_input))?;
    assert!(output.contains("Alice"));
    assert!(output.contains("Bob"));

    Ok(())
}

#[test]
fn test_conditional_formatting_with_json() -> Result<()> {
    let json_input = r#"[
        {"status": "active", "count": 100},
        {"status": "inactive", "count": 5}
    ]"#;

    // Test with row color formatting
    let output = run_tbl(
        &["--row-color", "1:green"],
        Some(json_input)
    )?;

    // Should still render the data (color codes are ANSI, harder to test)
    assert!(output.contains("active"));
    assert!(output.contains("inactive"));
    assert!(output.contains("100"));
    assert!(output.contains("5"));

    Ok(())
}

#[test]
fn test_empty_array() -> Result<()> {
    let json_input = "[]";

    let output = run_tbl(&[], Some(json_input))?;

    // Empty array should produce minimal output (just borders or empty)
    // The exact output depends on table rendering, but it shouldn't error
    assert!(output.len() < 50); // Should be very short

    Ok(())
}

#[test]
fn test_heterogeneous_objects() -> Result<()> {
    let json_input = r#"[
        {"name": "Alice", "age": 30, "city": "NYC"},
        {"name": "Bob", "age": 25},
        {"name": "Charlie", "city": "LA"}
    ]"#;

    let output = run_tbl(&[], Some(json_input))?;

    // All three keys should appear as columns
    assert!(output.contains("Alice"));
    assert!(output.contains("Bob"));
    assert!(output.contains("Charlie"));
    assert!(output.contains("30"));
    assert!(output.contains("25"));
    assert!(output.contains("NYC"));
    assert!(output.contains("LA"));

    Ok(())
}

#[test]
fn test_nested_objects_stringified() -> Result<()> {
    let json_input = r#"[
        {"name": "Alice", "address": {"city": "NYC", "zip": "10001"}}
    ]"#;

    let output = run_tbl(&[], Some(json_input))?;

    // Nested object should be stringified as JSON
    assert!(output.contains("Alice"));
    // The exact format may vary, but it should contain the nested data
    assert!(output.contains("NYC") || output.contains("city"));

    Ok(())
}
