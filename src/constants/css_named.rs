use std::collections::HashMap;

pub fn css_named_colors() -> HashMap<String, String> {
    let mut colors = HashMap::new();

    // CSS named colors
    colors.insert("red".to_string(), "oklch(0.637 0.237 25.331)".to_string());
    colors.insert(
        "orange".to_string(),
        "oklch(0.705 0.213 47.604)".to_string(),
    );
    colors.insert(
        "yellow".to_string(),
        "oklch(0.795 0.184 86.047)".to_string(),
    );
    colors.insert(
        "green".to_string(),
        "oklch(0.723 0.219 149.579)".to_string(),
    );
    colors.insert("blue".to_string(), "oklch(0.623 0.214 259.815)".to_string());
    colors.insert("purple".to_string(), "oklch(0.627 0.265 303.9)".to_string());
    colors.insert("pink".to_string(), "oklch(0.656 0.241 354.308)".to_string());
    colors.insert("black".to_string(), "#000000".to_string());
    colors.insert("white".to_string(), "#ffffff".to_string());
    colors.insert("gray".to_string(), "oklch(0.551 0.027 264.364)".to_string());
    colors.insert(
        "silver".to_string(),
        "oklch(0.707 0.022 261.325)".to_string(),
    );

    colors
}
