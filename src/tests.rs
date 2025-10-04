#[cfg(test)]
mod tests {
    use crate::functions::*;
    use crate::resolve_colors;
    use crate::ColorDefinitions;
    use crate::ResolvedColors;
    use std::collections::HashMap;

    #[test]
    fn test_resolve_simple_colors() {
        let definitions = ColorDefinitions(
            [
                ("primary".to_string(), "blue-500".to_string()),
                ("secondary".to_string(), "red".to_string()),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.get("primary").unwrap().starts_with("#"));
        assert!(resolved.0.get("secondary").unwrap().starts_with("#"));
    }

    #[test]
    fn test_resolve_function_calls() {
        let definitions = ColorDefinitions(
            [
                ("primary".to_string(), "blue-500".to_string()),
                ("darker".to_string(), "darken(primary, 20%)".to_string()),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.get("darker").unwrap().starts_with("#"));
    }

    #[test]
    fn test_unresolved_references() {
        let definitions =
            ColorDefinitions([("primary".to_string(), "darken(unknown, 10%)".to_string())].into());
        let resolved = resolve_colors(definitions);
        assert_eq!(resolved.0.get("primary").unwrap(), "UNRESOLVED_COLOR");
    }

    #[test]
    fn test_chained_references() {
        let definitions = ColorDefinitions(
            [
                ("base".to_string(), "blue-500".to_string()),
                ("mid".to_string(), "lighten(base, 10%)".to_string()),
                ("top".to_string(), "darken(mid, 5%)".to_string()),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.get("top").unwrap().starts_with("#"));
    }

    // Test color functions
    #[test]
    fn test_darken_function() {
        let result = darken("#3b82f6", "20%").unwrap();
        assert!(result.starts_with("#"));
        // Darkened blue should be darker
        let original = parse_color("#3b82f6").unwrap();
        let darkened = parse_color(&result).unwrap();
        assert!(darkened.l < original.l);
    }

    #[test]
    fn test_lighten_function() {
        let result = lighten("#3b82f6", "20%").unwrap();
        assert!(result.starts_with("#"));
        let original = parse_color("#3b82f6").unwrap();
        let lightened = parse_color(&result).unwrap();
        assert!(lightened.l > original.l);
    }

    #[test]
    fn test_temperature_up() {
        let result = temperature_up("#3b82f6", "30").unwrap();
        assert!(result.starts_with("#"));
        // Verify it's different from original
        assert_ne!(result, "#3b82f6");
    }

    #[test]
    fn test_temperature_down() {
        let result = temperature_down("#3b82f6", "30").unwrap();
        assert!(result.starts_with("#"));
        assert_ne!(result, "#3b82f6");
    }

    #[test]
    fn test_saturate() {
        let result = saturate("#808080", "0.2").unwrap();
        assert!(result.starts_with("#"));
        let original = parse_color("#808080").unwrap();
        let saturated = parse_color(&result).unwrap();
        assert!(saturated.chroma > original.chroma);
    }

    #[test]
    fn test_desaturate() {
        let result = desaturate("#ff0000", "0.5").unwrap();
        assert!(result.starts_with("#"));
        let original = parse_color("#ff0000").unwrap();
        let desaturated = parse_color(&result).unwrap();
        assert!(desaturated.chroma < original.chroma);
    }

    // Test parsing
    #[test]
    fn test_parse_hex_color() {
        let oklch = parse_color("#ff0000").unwrap();
        assert!((oklch.l - 0.637).abs() < 0.01);
        // Chroma and hue may vary slightly due to conversion
    }

    #[test]
    fn test_parse_oklch_color() {
        let oklch = parse_color("oklch(0.5 0.1 120)").unwrap();
        assert_eq!(oklch.l, 0.5);
        assert_eq!(oklch.chroma, 0.1);
        assert_eq!(oklch.hue, 120.0);
    }

    #[test]
    fn test_color_to_hex() {
        let oklch = palette::Oklch::new(0.5, 0.1, 120.0);
        let hex = color_to_hex(oklch);
        assert!(hex.starts_with("#"));
        assert_eq!(hex.len(), 7);
    }

    // Test resolver edge cases
    #[test]
    fn test_circular_reference() {
        let definitions = ColorDefinitions(
            [
                ("a".to_string(), "b".to_string()),
                ("b".to_string(), "a".to_string()),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        // Circular references are resolved to some value in the cycle
        assert!(resolved.0.get("a").unwrap() != "UNRESOLVED_COLOR");
        assert!(resolved.0.get("b").unwrap() != "UNRESOLVED_COLOR");
    }

    #[test]
    fn test_self_reference() {
        let definitions =
            ColorDefinitions([("a".to_string(), "darken(a, 10%)".to_string())].into());
        let resolved = resolve_colors(definitions);
        assert_eq!(resolved.0.get("a").unwrap(), "UNRESOLVED_COLOR");
    }

    #[test]
    fn test_mixed_formats() {
        let definitions = ColorDefinitions(
            [
                ("hex".to_string(), "#ff0000".to_string()),
                ("named".to_string(), "blue".to_string()),
                ("tailwind".to_string(), "red-500".to_string()),
                ("derived".to_string(), "lighten(hex, 10%)".to_string()),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.get("hex").unwrap().starts_with("#"));
        assert!(resolved.0.get("named").unwrap().starts_with("#"));
        assert!(resolved.0.get("tailwind").unwrap().starts_with("#"));
        assert!(resolved.0.get("derived").unwrap().starts_with("#"));
    }

    #[test]
    fn test_invalid_function() {
        let definitions = ColorDefinitions(
            [("invalid".to_string(), "unknown_func(color, 10)".to_string())].into(),
        );
        let resolved = resolve_colors(definitions);
        assert_eq!(resolved.0.get("invalid").unwrap(), "UNRESOLVED_COLOR");
    }

    #[test]
    fn test_literal_color() {
        let definitions = ColorDefinitions([("literal".to_string(), "#123456".to_string())].into());
        let resolved = resolve_colors(definitions);
        assert_eq!(resolved.0.get("literal").unwrap(), "#123456");
    }

    #[test]
    fn test_percentage_parsing() {
        let result = darken("#ffffff", "50%").unwrap();
        let original = parse_color("#ffffff").unwrap();
        let darkened = parse_color(&result).unwrap();
        assert!((darkened.l - (original.l - 0.5)).abs() < 0.01);
    }

    #[test]
    fn test_numeric_amount() {
        let result = temperature_up("#ff0000", "45").unwrap();
        assert!(result.starts_with("#"));
        assert_ne!(result, "#ff0000");
    }

    #[test]
    fn test_deeply_nested_functions() {
        let definitions = ColorDefinitions(
            [
                ("base".to_string(), "blue-500".to_string()),
                ("level1".to_string(), "darken(base, 10%)".to_string()),
                ("level2".to_string(), "lighten(level1, 5%)".to_string()),
                ("level3".to_string(), "saturate(level2, 0.1)".to_string()),
                (
                    "level4".to_string(),
                    "temperature_up(level3, 20)".to_string(),
                ),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.get("level4").unwrap().starts_with("#"));
    }

    #[test]
    fn test_mix_function() {
        let definitions = ColorDefinitions(
            [
                ("red".to_string(), "#ff0000".to_string()),
                ("blue".to_string(), "#0000ff".to_string()),
                ("purple".to_string(), "mix(red, blue, 0.5)".to_string()),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.get("purple").unwrap().starts_with("#"));
    }

    #[test]
    fn test_mix_pipe_syntax() {
        let definitions = ColorDefinitions(
            [
                ("red".to_string(), "#ff0000".to_string()),
                ("blue".to_string(), "#0000ff".to_string()),
                ("purple".to_string(), "red | mix(blue, 0.5)".to_string()),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.get("purple").unwrap().starts_with("#"));
    }

    #[test]
    fn test_hex_color_preservation() {
        let definitions =
            ColorDefinitions([("hex_color".to_string(), "#ff5733".to_string())].into());
        let resolved = resolve_colors(definitions);
        assert_eq!(resolved.0.get("hex_color").unwrap(), "#ff5733");
    }

    #[test]
    fn test_oklch_color_conversion() {
        let definitions = ColorDefinitions(
            [("oklch_color".to_string(), "oklch(0.6 0.2 180)".to_string())].into(),
        );
        let resolved = resolve_colors(definitions);
        let result = resolved.0.get("oklch_color").unwrap();
        assert!(result.starts_with("#"));
        // Verify it's different from a default color
        assert_ne!(result, "#000000");
    }

    #[test]
    fn test_boundary_lightness_values() {
        // Test darkening to black
        let result = darken("#ffffff", "100%").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.l < 0.01); // Should be very close to black

        // Test lightening to white
        let result = lighten("#000000", "100%").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.l > 0.99); // Should be very close to white
    }

    #[test]
    fn test_boundary_chroma_values() {
        // Test desaturating to grayscale
        let result = desaturate("#ff0000", "1.0").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.chroma < 0.01); // Should be very close to grayscale

        // Test saturating
        let result = saturate("#808080", "0.5").unwrap();
        let color = parse_color(&result).unwrap();
        assert!(color.chroma > 0.0); // Should have some saturation
    }

    #[test]
    fn test_large_temperature_changes() {
        let result = temperature_up("#ff0000", "360").unwrap();
        assert!(result.starts_with("#"));
        // Should cycle through the color wheel
    }

    #[test]
    fn test_negative_temperature_changes() {
        let result = temperature_down("#ff0000", "45").unwrap();
        assert!(result.starts_with("#"));
        assert_ne!(result, "#ff0000");
    }

    #[test]
    fn test_mixed_case_color_names() {
        let definitions = ColorDefinitions([("color".to_string(), "BLUE-500".to_string())].into());
        let resolved = resolve_colors(definitions);
        // Constants are case-insensitive
        assert!(resolved.0.get("color").unwrap().starts_with("#"));
    }

    #[test]
    fn test_invalid_hex_colors() {
        let definitions = ColorDefinitions([("bad_hex".to_string(), "#xyz".to_string())].into());
        let resolved = resolve_colors(definitions);
        // Invalid hex should be treated as literal but might not resolve properly
        // This depends on parser behavior
        assert_eq!(resolved.0.get("bad_hex").unwrap(), "#xyz");
    }

    #[test]
    fn test_empty_definitions() {
        let definitions = ColorDefinitions(HashMap::new());
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.is_empty());
    }

    #[test]
    fn test_single_definition() {
        let definitions = ColorDefinitions([("only".to_string(), "red".to_string())].into());
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.get("only").unwrap().starts_with("#"));
        assert_eq!(resolved.0.len(), 1);
    }

    #[test]
    fn test_function_with_invalid_color() {
        let definitions =
            ColorDefinitions([("bad".to_string(), "darken(invalid, 10%)".to_string())].into());
        let resolved = resolve_colors(definitions);
        assert_eq!(resolved.0.get("bad").unwrap(), "UNRESOLVED_COLOR");
    }

    #[test]
    fn test_function_with_invalid_percentage() {
        let definitions =
            ColorDefinitions([("bad".to_string(), "darken(#ff0000, invalid%)".to_string())].into());
        let resolved = resolve_colors(definitions);
        assert_eq!(resolved.0.get("bad").unwrap(), "UNRESOLVED_COLOR");
    }

    #[test]
    fn test_multiple_unresolved_dependencies() {
        let definitions = ColorDefinitions(
            [
                ("a".to_string(), "unknown1".to_string()),
                ("b".to_string(), "darken(a, 10%)".to_string()),
                ("c".to_string(), "lighten(b, 5%)".to_string()),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        assert_eq!(resolved.0.get("a").unwrap(), "UNRESOLVED_COLOR");
        assert_eq!(resolved.0.get("b").unwrap(), "UNRESOLVED_COLOR");
        assert_eq!(resolved.0.get("c").unwrap(), "UNRESOLVED_COLOR");
    }

    #[test]
    fn test_partial_resolution() {
        let definitions = ColorDefinitions(
            [
                ("good".to_string(), "red".to_string()),
                ("bad".to_string(), "unknown".to_string()),
                ("derived".to_string(), "darken(good, 10%)".to_string()),
            ]
            .into(),
        );
        let resolved = resolve_colors(definitions);
        assert!(resolved.0.get("good").unwrap().starts_with("#"));
        assert_eq!(resolved.0.get("bad").unwrap(), "UNRESOLVED_COLOR");
        assert!(resolved.0.get("derived").unwrap().starts_with("#"));
    }
}
