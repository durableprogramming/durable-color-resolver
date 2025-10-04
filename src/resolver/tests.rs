use super::*;
use crate::ColorDefinitions;
use std::collections::HashMap;

#[test]
fn test_resolve_empty_definitions() {
    let definitions = ColorDefinitions(HashMap::new());
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.is_empty());
}

#[test]
fn test_resolve_single_constant() {
    let definitions = ColorDefinitions([("primary".to_string(), "red-500".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("primary").unwrap().starts_with('#'));
    assert_eq!(resolved.0.len(), 1);
}

#[test]
fn test_resolve_hex_literal() {
    let definitions = ColorDefinitions([("color".to_string(), "#ff0000".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("color").unwrap(), "#ff0000");
}

#[test]
fn test_resolve_oklch_literal() {
    let definitions =
        ColorDefinitions([("color".to_string(), "oklch(0.5 0.1 120)".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("color").unwrap().starts_with('#'));
}

#[test]
fn test_resolve_simple_reference() {
    let definitions = ColorDefinitions(
        [
            ("base".to_string(), "blue-500".to_string()),
            ("primary".to_string(), "base".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("base").unwrap().starts_with('#'));
    assert_eq!(
        resolved.0.get("primary").unwrap(),
        resolved.0.get("base").unwrap()
    );
}

#[test]
fn test_resolve_function_call() {
    let definitions = ColorDefinitions(
        [
            ("base".to_string(), "red-500".to_string()),
            ("lighter".to_string(), "lighten(base, 20%)".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("lighter").unwrap().starts_with('#'));
    // Lighter should be different from base
    assert_ne!(
        resolved.0.get("lighter").unwrap(),
        resolved.0.get("base").unwrap()
    );
}

#[test]
fn test_resolve_chained_functions() {
    let definitions = ColorDefinitions(
        [
            ("base".to_string(), "#808080".to_string()),
            ("step1".to_string(), "lighten(base, 10%)".to_string()),
            ("step2".to_string(), "saturate(step1, 0.2)".to_string()),
            ("final".to_string(), "darken(step2, 5%)".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("final").unwrap().starts_with('#'));
}

#[test]
fn test_resolve_multiple_levels() {
    let definitions = ColorDefinitions(
        [
            ("level0".to_string(), "green-500".to_string()),
            ("level1".to_string(), "level0".to_string()),
            ("level2".to_string(), "level1".to_string()),
            ("level3".to_string(), "darken(level2, 10%)".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("level3").unwrap().starts_with('#'));
    assert_eq!(
        resolved.0.get("level0").unwrap(),
        resolved.0.get("level1").unwrap()
    );
    assert_eq!(
        resolved.0.get("level1").unwrap(),
        resolved.0.get("level2").unwrap()
    );
}

#[test]
fn test_resolve_mixed_types() {
    let definitions = ColorDefinitions(
        [
            ("hex".to_string(), "#ff5733".to_string()),
            ("named".to_string(), "blue".to_string()),
            ("tailwind".to_string(), "purple-400".to_string()),
            ("derived".to_string(), "temperature_up(hex, 30)".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("hex").unwrap(), "#ff5733");
    assert!(resolved.0.get("named").unwrap().starts_with('#'));
    assert!(resolved.0.get("tailwind").unwrap().starts_with('#'));
    assert!(resolved.0.get("derived").unwrap().starts_with('#'));
}

#[test]
fn test_resolve_circular_reference() {
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
fn test_resolve_self_reference() {
    let definitions =
        ColorDefinitions([("self".to_string(), "darken(self, 10%)".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("self").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_unresolved_reference() {
    let definitions = ColorDefinitions([("bad".to_string(), "nonexistent".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("bad").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_invalid_function() {
    let definitions =
        ColorDefinitions([("bad".to_string(), "invalid_func(#ff0000)".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("bad").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_invalid_color_literal() {
    let definitions = ColorDefinitions([("bad".to_string(), "notacolor".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("bad").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_case_insensitive_constants() {
    let definitions = ColorDefinitions([("color".to_string(), "RED".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("color").unwrap().starts_with('#'));
}

#[test]
fn test_resolve_case_insensitive_references() {
    let definitions = ColorDefinitions(
        [
            ("primary".to_string(), "blue-500".to_string()),
            ("secondary".to_string(), "PRIMARY".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert_eq!(
        resolved.0.get("secondary").unwrap(),
        resolved.0.get("primary").unwrap()
    );
}

#[test]
fn test_resolve_complex_dependencies() {
    let definitions = ColorDefinitions(
        [
            ("a".to_string(), "red-500".to_string()),
            ("b".to_string(), "lighten(a, 10%)".to_string()),
            ("c".to_string(), "darken(b, 5%)".to_string()),
            ("d".to_string(), "c".to_string()),
            ("e".to_string(), "temperature_up(d, 20)".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("e").unwrap().starts_with('#'));
    assert_eq!(resolved.0.len(), 5);
}

#[test]
fn test_resolve_partial_resolution() {
    let definitions = ColorDefinitions(
        [
            ("good".to_string(), "green-500".to_string()),
            ("bad".to_string(), "invalid_color".to_string()),
            ("derived".to_string(), "saturate(good, 0.1)".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("good").unwrap().starts_with('#'));
    assert_eq!(resolved.0.get("bad").unwrap(), "UNRESOLVED_COLOR");
    assert!(resolved.0.get("derived").unwrap().starts_with('#'));
}

#[test]
fn test_resolve_all_functions() {
    let definitions = ColorDefinitions(
        [
            ("base".to_string(), "#ff0000".to_string()),
            ("darkened".to_string(), "darken(base, 20%)".to_string()),
            ("lightened".to_string(), "lighten(base, 15%)".to_string()),
            ("warmer".to_string(), "temperature_up(base, 45)".to_string()),
            (
                "cooler".to_string(),
                "temperature_down(base, 30)".to_string(),
            ),
            (
                "more_saturated".to_string(),
                "saturate(base, 0.2)".to_string(),
            ),
            (
                "less_saturated".to_string(),
                "desaturate(base, 0.3)".to_string(),
            ),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    for key in [
        "base",
        "darkened",
        "lightened",
        "warmer",
        "cooler",
        "more_saturated",
        "less_saturated",
    ] {
        assert!(resolved.0.get(key).unwrap().starts_with('#'));
    }
}

#[test]
fn test_resolve_nested_function_calls() {
    let definitions = ColorDefinitions(
        [
            ("base".to_string(), "blue-500".to_string()),
            (
                "complex".to_string(),
                "darken(lighten(temperature_up(base, 20), 10%), 5%)".to_string(),
            ),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("complex").unwrap().starts_with('#'));
}

#[test]
fn test_resolve_large_definition_set() {
    let mut definitions = HashMap::new();
    for i in 0..50 {
        let key = format!("color{}", i);
        let value = if i == 0 {
            "red-500".to_string()
        } else {
            format!("lighten(color{}, 2%)", i - 1)
        };
        definitions.insert(key, value);
    }
    let definitions = ColorDefinitions(definitions);
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.len(), 50);
    for i in 0..50 {
        let key = format!("color{}", i);
        assert!(resolved.0.get(&key).unwrap().starts_with('#'));
    }
}

#[test]
fn test_resolve_empty_string_definition() {
    let definitions = ColorDefinitions([("empty".to_string(), "".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("empty").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_whitespace_only_definition() {
    let definitions = ColorDefinitions([("ws".to_string(), "   ".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("ws").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_numeric_literal() {
    let definitions = ColorDefinitions([("num".to_string(), "123".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("num").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_malformed_oklch() {
    let definitions = ColorDefinitions([("bad".to_string(), "oklch(invalid)".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("bad").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_function_with_invalid_args() {
    let definitions =
        ColorDefinitions([("bad".to_string(), "darken(#ff0000, invalid%)".to_string())].into());
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("bad").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_reference_to_unresolved() {
    let definitions = ColorDefinitions(
        [
            ("a".to_string(), "b".to_string()),
            ("b".to_string(), "unknown".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.get("a").unwrap(), "UNRESOLVED_COLOR");
    assert_eq!(resolved.0.get("b").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_complex_circular_with_functions() {
    let definitions = ColorDefinitions(
        [
            ("a".to_string(), "lighten(b, 10%)".to_string()),
            ("b".to_string(), "darken(a, 5%)".to_string()),
        ]
        .into(),
    );
    let resolved = resolve_colors(definitions);
    // Since they depend on each other, they remain unresolved
    assert_eq!(resolved.0.get("a").unwrap(), "UNRESOLVED_COLOR");
    assert_eq!(resolved.0.get("b").unwrap(), "UNRESOLVED_COLOR");
}

#[test]
fn test_resolve_order_independence() {
    // Test that resolution works regardless of definition order
    let definitions1 = ColorDefinitions(
        [
            ("first".to_string(), "red-500".to_string()),
            ("second".to_string(), "first".to_string()),
        ]
        .into(),
    );
    let definitions2 = ColorDefinitions(
        [
            ("second".to_string(), "first".to_string()),
            ("first".to_string(), "red-500".to_string()),
        ]
        .into(),
    );
    let resolved1 = resolve_colors(definitions1);
    let resolved2 = resolve_colors(definitions2);
    assert_eq!(resolved1.0, resolved2.0);
}

#[test]
fn test_resolve_duplicate_keys() {
    // HashMap will overwrite, so last one wins
    let mut map = HashMap::new();
    map.insert("dup".to_string(), "red-500".to_string());
    map.insert("dup".to_string(), "blue-500".to_string());
    let definitions = ColorDefinitions(map);
    let resolved = resolve_colors(definitions);
    assert!(resolved.0.get("dup").unwrap().starts_with('#'));
    // Should be blue, but we can't test the exact value easily
}

#[test]
fn test_resolve_max_iterations() {
    // Test with many interdependent definitions
    let mut definitions = HashMap::new();
    for i in 0..100 {
        let key = format!("c{}", i);
        let value = if i == 0 {
            "red-500".to_string()
        } else {
            format!("c{}", i - 1)
        };
        definitions.insert(key, value);
    }
    let definitions = ColorDefinitions(definitions);
    let resolved = resolve_colors(definitions);
    assert_eq!(resolved.0.len(), 100);
    // All should resolve since it's a chain, not a cycle
    for i in 0..100 {
        let key = format!("c{}", i);
        assert!(resolved.0.get(&key).unwrap().starts_with('#'));
        if i > 0 {
            let prev_key = format!("c{}", i - 1);
            assert_eq!(
                resolved.0.get(&key).unwrap(),
                resolved.0.get(&prev_key).unwrap()
            );
        }
    }
}
