use durable_color_resolver::{parser, ColorDefinitions};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};

pub fn determine_input_mode(input: Option<&str>) -> (bool, String) {
    if let Some(input) = input {
        // Check if it's a file that exists or "-"
        if input == "-" || std::path::Path::new(input).exists() {
            (false, input.to_string())
        } else {
            // Treat as expression
            (true, input.to_string())
        }
    } else {
        // No input provided
        (false, "".to_string()) // This will be handled as error in main
    }
}

pub fn read_input(input_source: &str) -> Result<String, Box<dyn std::error::Error>> {
    if input_source == "-" {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(buffer)
    } else {
        fs::read_to_string(input_source).map_err(Into::into)
    }
}

pub fn parse_and_validate_json(
    json_str: &str,
) -> Result<ColorDefinitions, Box<dyn std::error::Error>> {
    // First, try to parse as a generic JSON value to provide better error messages
    let json_value: serde_json::Value = serde_json::from_str(json_str)?;

    // Validate that it's an object
    match &json_value {
        serde_json::Value::Object(obj) => {
            // Validate that all values are strings
            for (key, value) in obj {
                if !value.is_string() {
                    return Err(format!("All color values must be strings. Key '{key}' has value '{value}' which is not a string.").into());
                }
            }
        }
        _ => {
            return Err("Input JSON must be an object (key-value pairs).".into());
        }
    };

    // Now parse as ColorDefinitions
    let definitions: ColorDefinitions = serde_json::from_str(json_str)?;
    Ok(definitions)
}

pub fn parse_expression(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let color_map = HashMap::new();
    parser::parse_expression(input, &color_map).map_err(Into::into)
}
