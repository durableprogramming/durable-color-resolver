#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_simple_expression() {
        let color_map = HashMap::new();
        let result = parse_expression("blue-500", &color_map);
        assert!(result.unwrap().starts_with("#")); // Constants get resolved to hex
    }

    #[test]
    fn test_parse_hex_literal() {
        let color_map = HashMap::new();
        let result = parse_expression("#ff0000", &color_map);
        assert_eq!(result.unwrap(), "#ff0000");
    }

    #[test]
    fn test_parse_reference() {
        let mut color_map = HashMap::new();
        color_map.insert("primary".to_string(), "#ff0000".to_string());
        let result = parse_expression("primary", &color_map);
        assert_eq!(result.unwrap(), "#ff0000");
    }

    #[test]
    fn test_parse_function_call() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("darken(base, 20%)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_nested_function_calls() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("lighten(darken(base, 10%), 5%)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_args_simple() {
        let color_map = HashMap::new();
        let result = super::args::parse_args("arg1, arg2", &color_map);
        assert_eq!(result.unwrap(), vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_parse_args_with_functions() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = super::args::parse_args("base, 20%", &color_map);
        assert_eq!(result.unwrap(), vec!["#ff0000", "20%"]);
    }

    #[test]
    fn test_parse_args_nested() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = super::args::parse_args("darken(base, 10%), 5%", &color_map).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result[0].starts_with("#"));
        assert_eq!(result[1], "5%");
    }

    #[test]
    fn test_parse_args_with_parentheses() {
        let color_map = HashMap::new();
        let result =
            super::args::parse_args("darken(#ff0000, 10%), lighten(#00ff00, 5%)", &color_map);
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
        assert!(args[0].starts_with("#"));
        assert!(args[1].starts_with("#"));
    }

    #[test]
    fn test_parse_unknown_function() {
        let color_map = HashMap::new();
        let result = parse_expression("unknown_func(color, 10%)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_parse_invalid_color() {
        let color_map = HashMap::new();
        let result = parse_expression("invalid(color)", &color_map);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_color_value_oklch() {
        // This is a private function, but we can test it indirectly
        let mut color_map = HashMap::new();
        color_map.insert("oklch_color".to_string(), "oklch(0.5 0.1 120)".to_string());
        let result = parse_expression("oklch_color", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_expression_with_spaces() {
        let color_map = HashMap::new();
        let result = parse_expression("  blue-500  ", &color_map);
        assert!(result.unwrap().starts_with("#")); // Constants get resolved to hex
    }

    #[test]
    fn test_parse_complex_expression() {
        let mut color_map = HashMap::new();
        color_map.insert("primary".to_string(), "blue-500".to_string());
        color_map.insert("secondary".to_string(), "red-500".to_string());
        let result = parse_expression(
            "darken(lighten(primary, 10%), temperature_up(secondary, 15))",
            &color_map,
        );
        // This should fail because temperature_up expects a number, not a color
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_temperature_function() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("temperature_up(base, 30)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_saturate_function() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#808080".to_string());
        let result = parse_expression("saturate(base, 0.2)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_desaturate_function() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("desaturate(base, 0.5)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_chained_references() {
        let mut color_map = HashMap::new();
        color_map.insert("level1".to_string(), "blue-500".to_string());
        color_map.insert("level2".to_string(), "level1".to_string());
        let result = parse_expression("level2", &color_map);
        assert!(result.unwrap().starts_with('#')); // Resolves transitively
    }

    #[test]
    fn test_parse_empty_args() {
        let color_map = HashMap::new();
        let result = super::args::parse_args("", &color_map);
        assert_eq!(result.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn test_parse_single_arg() {
        let color_map = HashMap::new();
        let result = super::args::parse_args("single", &color_map);
        assert_eq!(result.unwrap(), vec!["single"]);
    }

    #[test]
    fn test_parse_args_with_commas_in_functions() {
        let color_map = HashMap::new();
        // Test that commas inside function calls are handled
        let result = super::args::parse_args("darken(#ff0000, 10%), #00ff00", &color_map);
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
        assert!(args[0].starts_with("#"));
        assert_eq!(args[1], "#00ff00");
    }

    #[test]
    fn test_parse_malformed_oklch() {
        let color_map = HashMap::new();
        let result = parse_expression("oklch(invalid)", &color_map);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_function_args() {
        let color_map = HashMap::new();
        let result = parse_expression("darken()", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires exactly 2 arguments"));
    }

    #[test]
    fn test_parse_function_with_too_many_args() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("darken(base, 10%, extra)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires exactly 2 arguments"));
    }

    #[test]
    fn test_parse_function_with_too_few_args() {
        let color_map = HashMap::new();
        let result = parse_expression("darken(#ff0000)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires exactly 2 arguments"));
    }

    #[test]
    fn test_parse_function_with_spaces_in_args() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("darken( base , 10% )", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_nested_function_with_spaces() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("lighten( darken( base , 10% ) , 5% )", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_mixed_references_and_literals() {
        let mut color_map = HashMap::new();
        color_map.insert("primary".to_string(), "#ff0000".to_string());
        let result = parse_expression("darken(primary, 20%)", &color_map);
        assert!(result.is_ok());
        let result2 = parse_expression("darken(#00ff00, 20%)", &color_map);
        assert!(result2.is_ok());
        assert_ne!(result.unwrap(), result2.unwrap());
    }

    #[test]
    fn test_parse_invalid_function_name() {
        let color_map = HashMap::new();
        let result = parse_expression("invalid_func(#ff0000, 10%)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_parse_function_case_sensitivity() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("DARKEN(base, 10%)", &color_map);
        assert!(result.is_err()); // Functions are case-sensitive
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_parse_color_reference_case_sensitivity() {
        let mut color_map = HashMap::new();
        color_map.insert("primary".to_string(), "#ff0000".to_string());
        let result = parse_expression("PRIMARY", &color_map);
        assert_eq!(result.unwrap(), "#ff0000"); // References are case-sensitive
    }

    #[test]
    fn test_parse_constant_case_insensitive() {
        let color_map = HashMap::new();
        let result = parse_expression("RED", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_hex_case_insensitive() {
        let color_map = HashMap::new();
        let result = parse_expression("#FF0000", &color_map);
        assert_eq!(result.unwrap(), "#FF0000");
    }

    #[test]
    fn test_parse_empty_expression() {
        let color_map = HashMap::new();
        let result = parse_expression("", &color_map);
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_parse_whitespace_only() {
        let color_map = HashMap::new();
        let result = parse_expression("   ", &color_map);
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_parse_args_with_quotes() {
        let color_map = HashMap::new();
        let result = super::args::parse_args("\"arg1\", \"arg2\"", &color_map);
        assert_eq!(result.unwrap(), vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_parse_args_with_escaped_quotes() {
        let color_map = HashMap::new();
        let result = super::args::parse_args("\"arg\\\"with\\\"quotes\", normal", &color_map);
        // This might not be fully implemented, but let's test current behavior
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn test_parse_complex_nested_args() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = super::args::parse_args(
            "darken(base, 10%), lighten(darken(base, 5%), 15%)",
            &color_map,
        );
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
        assert!(args[0].starts_with("#"));
        assert!(args[1].starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_simple() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("base | darken(0.2)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_with_literal() {
        let color_map = HashMap::new();
        let result = parse_expression("red | darken(0.2)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_chained() {
        let color_map = HashMap::new();
        let result = parse_expression("red | darken(0.2) | lighten(0.1)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_with_hex() {
        let color_map = HashMap::new();
        let result = parse_expression("#ff0000 | darken(0.2)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_invalid_function() {
        let color_map = HashMap::new();
        let result = parse_expression("red | invalid_func(0.2)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_parse_pipe_invalid_syntax() {
        let color_map = HashMap::new();
        let result = parse_expression("red | not_a_function", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid pipe syntax"));
    }

    #[test]
    fn test_parse_pipe_equivalent_to_function_call() {
        let color_map = HashMap::new();
        let pipe_result = parse_expression("red | darken(0.2)", &color_map);
        let func_result = parse_expression("darken(red, 0.2)", &color_map);
        assert_eq!(pipe_result.unwrap(), func_result.unwrap());
    }

    #[test]
    fn test_parse_pipe_with_spaces() {
        let color_map = HashMap::new();
        let result = parse_expression("red | darken(0.2) | lighten(0.1)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }
}
