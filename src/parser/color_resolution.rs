use crate::functions::*;

pub fn resolve_color_value(color_str: &str) -> Result<String, String> {
    if color_str.starts_with("oklch(") {
        parse_color(color_str).map(color_to_hex)
    } else {
        Ok(color_str.to_string())
    }
}
