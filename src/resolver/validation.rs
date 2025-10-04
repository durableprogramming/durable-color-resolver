use crate::functions::parse_color;
use crate::ColorDefinitions;

pub fn is_valid_color_value(value: &str, definitions: &ColorDefinitions) -> bool {
    value.starts_with('#')
        || value.starts_with("oklch(")
        || parse_color(value).is_ok()
        || definitions.0.contains_key(value)
}
