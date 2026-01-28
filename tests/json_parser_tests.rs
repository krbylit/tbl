use tbl::json_parser::*;

#[cfg(test)]
mod format_detection {
    use super::*;

    #[test]
    fn test_detect_array_of_objects() {
        let input = r#"[{"name": "Alice"}]"#;
        assert_eq!(FormatDetector::detect(input), InputFormat::Json);
    }

    #[test]
    fn test_detect_single_object() {
        let input = r#"{"name": "Alice"}"#;
        assert_eq!(FormatDetector::detect(input), InputFormat::Json);
    }

    #[test]
    fn test_detect_csv_fallback() {
        let input = "name,age\nAlice,30";
        assert_eq!(FormatDetector::detect(input), InputFormat::Csv);
    }

    #[test]
    fn test_detect_with_leading_whitespace() {
        let input = "  \n  [{}]";
        assert_eq!(FormatDetector::detect(input), InputFormat::Json);
    }
}

#[cfg(test)]
mod depth_limit {
    use super::*;

    #[test]
    fn test_depth_within_limit() {
        let input = r#"{"a": {"b": {"c": "value"}}}"#;
        let result = parse_with_depth_limit(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_depth_exceeds_limit() {
        // serde_json has built-in recursion limit of 128
        // Test that deeply nested JSON fails gracefully
        // Use 150 levels to ensure it exceeds serde_json's limit
        let mut json = String::from("\"value\"");
        for _ in 0..150 {
            json = format!(r#"{{"x":{}}}"#, json);
        }

        let result = parse_with_depth_limit(&json);
        assert!(result.is_err(), "Depth 150 should fail");
        let err_msg = result.unwrap_err().to_string();
        // Error comes from serde_json's parsing, not our check
        assert!(err_msg.contains("Failed to parse JSON"));
    }

    #[test]
    fn test_depth_within_reasonable_limit() {
        // Test that reasonably deep JSON (< 100 levels) works fine
        let mut json = String::from("\"value\"");
        for _ in 0..50 {
            json = format!(r#"{{"x":{}}}"#, json);
        }

        let result = parse_with_depth_limit(&json);
        assert!(result.is_ok(), "Depth 50 should pass");
    }
}

#[cfg(test)]
mod column_ordering {
    use super::*;

    #[test]
    fn test_first_occurrence_ordering() {
        let mut index = ColumnIndex::new();
        index.insert("name".to_string());
        index.insert("age".to_string());
        index.insert("email".to_string());

        let keys = index.keys();
        assert_eq!(keys, vec!["name", "age", "email"]);
    }

    #[test]
    fn test_duplicate_insertion_preserves_order() {
        let mut index = ColumnIndex::new();
        index.insert("name".to_string());
        index.insert("age".to_string());
        index.insert("name".to_string()); // Duplicate

        let keys = index.keys();
        assert_eq!(keys, vec!["name", "age"]);
        assert_eq!(index.index_of("name"), Some(0));
        assert_eq!(index.index_of("age"), Some(1));
    }

    #[test]
    fn test_index_lookup() {
        let mut index = ColumnIndex::new();
        index.insert("name".to_string());
        index.insert("age".to_string());

        assert_eq!(index.index_of("name"), Some(0));
        assert_eq!(index.index_of("age"), Some(1));
        assert_eq!(index.index_of("email"), None);
    }
}

#[cfg(test)]
mod value_conversion {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_convert_string() {
        let value = json!("Hello");
        assert_eq!(value_to_cell_string(&value), "Hello");
    }

    #[test]
    fn test_convert_number_integer() {
        let value = json!(42);
        assert_eq!(value_to_cell_string(&value), "42");
    }

    #[test]
    fn test_convert_number_float() {
        let value = json!(3.14);
        assert_eq!(value_to_cell_string(&value), "3.14");
    }

    #[test]
    fn test_convert_boolean_true() {
        let value = json!(true);
        assert_eq!(value_to_cell_string(&value), "true");
    }

    #[test]
    fn test_convert_boolean_false() {
        let value = json!(false);
        assert_eq!(value_to_cell_string(&value), "false");
    }

    #[test]
    fn test_convert_null() {
        let value = json!(null);
        assert_eq!(value_to_cell_string(&value), "");
    }

    #[test]
    fn test_convert_array() {
        let value = json!([1, 2, 3]);
        let result = value_to_cell_string(&value);
        assert!(result.contains('['));
        assert!(result.contains("1"));
        assert!(result.contains("2"));
        assert!(result.contains("3"));
    }

    #[test]
    fn test_convert_nested_object() {
        let value = json!({"key": "value"});
        let result = value_to_cell_string(&value);
        assert!(result.contains('{'));
        assert!(result.contains("key"));
        assert!(result.contains("value"));
    }
}

#[cfg(test)]
mod array_of_objects_parsing {
    use super::*;

    #[test]
    fn test_parse_basic_array() {
        let input = r#"[
            {"name": "Alice", "age": 30},
            {"name": "Bob", "age": 25}
        ]"#;

        let result = parse_with_depth_limit(input);
        assert!(result.is_ok());

        // This test will fail until we implement detect_json_format
        // TODO: Implement detect_json_format and table conversion
    }

    #[test]
    fn test_parse_empty_array() {
        let input = "[]";
        let result = parse_with_depth_limit(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_heterogeneous_objects() {
        let input = r#"[
            {"name": "Alice", "age": 30, "email": "alice@example.com"},
            {"name": "Bob", "age": 25},
            {"name": "Charlie", "email": "charlie@example.com"}
        ]"#;

        let result = parse_with_depth_limit(input);
        assert!(result.is_ok());

        // Columns should be: name, age, email (first occurrence order)
        // TODO: Implement and verify column ordering
    }

    #[test]
    fn test_parse_with_nested_values() {
        let input = r#"[
            {"name": "Alice", "address": {"city": "NYC"}},
            {"name": "Bob", "address": {"city": "LA"}}
        ]"#;

        let result = parse_with_depth_limit(input);
        assert!(result.is_ok());

        // Nested objects should be stringified
        // TODO: Verify nested object handling
    }

    #[test]
    fn test_parse_with_array_values() {
        let input = r#"[
            {"name": "Alice", "scores": [95, 87, 92]},
            {"name": "Bob", "scores": [88, 91]}
        ]"#;

        let result = parse_with_depth_limit(input);
        assert!(result.is_ok());

        // Arrays should be stringified
        // TODO: Verify array handling
    }

    #[test]
    fn test_parse_with_null_values() {
        let input = r#"[
            {"name": "Alice", "age": 30, "email": null},
            {"name": "Bob", "age": null, "email": "bob@example.com"}
        ]"#;

        let result = parse_with_depth_limit(input);
        assert!(result.is_ok());

        // Null values should become empty strings
        // TODO: Verify null handling
    }

    #[test]
    fn test_parse_mixed_types() {
        let input = r#"[
            {"id": 1, "name": "Alice", "active": true, "score": 95.5},
            {"id": 2, "name": "Bob", "active": false, "score": 87.3}
        ]"#;

        let result = parse_with_depth_limit(input);
        assert!(result.is_ok());

        // All types should convert to strings
        // TODO: Verify type conversion
    }
}
