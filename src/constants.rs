mod css_named;
mod tailwind;

use std::collections::HashMap;

pub fn color_constants() -> HashMap<String, String> {
    let mut colors = HashMap::new();

    // Merge Tailwind colors
    colors.extend(tailwind::tailwind_colors());

    // Merge CSS named colors
    colors.extend(css_named::css_named_colors());

    colors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_constants_contains_tailwind_colors() {
        let constants = color_constants();
        // Test some key colors
        assert!(constants.contains_key("red-500"));
        assert!(constants.contains_key("blue-500"));
        assert!(constants.contains_key("green-500"));
        assert!(constants.contains_key("yellow-500"));
        assert!(constants.contains_key("purple-500"));
        assert!(constants.contains_key("pink-500"));
    }

    #[test]
    fn test_color_constants_contains_css_colors() {
        let constants = color_constants();
        assert!(constants.contains_key("red"));
        assert!(constants.contains_key("blue"));
        assert!(constants.contains_key("green"));
        assert!(constants.contains_key("black"));
        assert!(constants.contains_key("white"));
        assert!(constants.contains_key("gray"));
        assert!(constants.contains_key("silver"));
    }

    #[test]
    fn test_color_constants_tailwind_shades() {
        let constants = color_constants();
        // Test that all shades are present for red
        for shade in [50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 950].iter() {
            let key = format!("red-{}", shade);
            assert!(constants.contains_key(&key), "Missing {}", key);
        }
    }

    #[test]
    fn test_color_constants_oklch_format() {
        let constants = color_constants();
        // Most colors should be in OKLCH format
        let red_500 = constants.get("red-500").unwrap();
        assert!(red_500.starts_with("oklch("));
        assert!(red_500.ends_with(")"));
    }

    #[test]
    fn test_color_constants_hex_format() {
        let constants = color_constants();
        // Black and white are hex
        assert_eq!(constants.get("black").unwrap(), "#000000");
        assert_eq!(constants.get("white").unwrap(), "#ffffff");
    }

    #[test]
    fn test_color_constants_count() {
        let constants = color_constants();
        // Should have many colors: 22 families * 11 shades + css colors
        // 22 families: red, orange, amber, yellow, lime, green, emerald, teal, cyan, sky, blue, indigo, violet, purple, fuchsia, pink, rose, slate, gray, zinc, neutral, stone
        // 11 shades each: 50,100,200,300,400,500,600,700,800,900,950
        // Plus CSS colors: red, orange, yellow, green, blue, purple, pink, black, white, gray, silver
        // Some overlap, but approximately 22*11 + 11 = 253
        assert!(
            constants.len() > 200,
            "Expected >200 colors, got {}",
            constants.len()
        );
    }

    #[test]
    fn test_color_constants_no_duplicates() {
        let constants = color_constants();
        let mut seen = std::collections::HashSet::new();
        for (key, _value) in &constants {
            assert!(!seen.contains(key), "Duplicate key: {}", key);
            seen.insert(key.clone());
            // Also check no duplicate values (though possible)
        }
    }

    #[test]
    fn test_color_constants_valid_oklch() {
        let constants = color_constants();
        for (key, value) in &constants {
            if value.starts_with("oklch(") {
                // Should be parseable
                assert!(
                    value.ends_with(")"),
                    "Invalid OKLCH format for {}: {}",
                    key,
                    value
                );
                let inner = &value[6..value.len() - 1];
                let parts: Vec<&str> = inner.split_whitespace().collect();
                assert_eq!(parts.len(), 3, "Invalid OKLCH parts for {}: {}", key, value);
                for part in &parts {
                    assert!(
                        part.parse::<f32>().is_ok(),
                        "Invalid number in OKLCH for {}: {}",
                        key,
                        part
                    );
                }
            }
        }
    }

    #[test]
    fn test_color_constants_case_sensitivity() {
        let constants = color_constants();
        // Keys should be lowercase
        for key in constants.keys() {
            assert_eq!(key, &key.to_lowercase(), "Key should be lowercase: {}", key);
        }
    }

    #[test]
    fn test_color_constants_specific_values() {
        let constants = color_constants();
        // Test some known values
        assert_eq!(
            constants.get("red-500").unwrap(),
            "oklch(0.637 0.237 25.331)"
        );
        assert_eq!(
            constants.get("blue-500").unwrap(),
            "oklch(0.623 0.214 259.815)"
        );
        assert_eq!(
            constants.get("green-500").unwrap(),
            "oklch(0.723 0.219 149.579)"
        );
        assert_eq!(constants.get("black").unwrap(), "#000000");
        assert_eq!(constants.get("white").unwrap(), "#ffffff");
    }

    #[test]
    fn test_color_constants_all_families_present() {
        let constants = color_constants();
        let families = [
            "red", "orange", "amber", "yellow", "lime", "green", "emerald", "teal", "cyan", "sky",
            "blue", "indigo", "violet", "purple", "fuchsia", "pink", "rose", "slate", "gray",
            "zinc", "neutral", "stone",
        ];
        for family in &families {
            let key = format!("{}-500", family);
            assert!(constants.contains_key(&key), "Missing family: {}", family);
        }
    }
}
