use durable_color_resolver::ResolvedColors;
use std::fs;

pub fn format_output(
    resolved: &ResolvedColors,
    compact: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    if compact {
        serde_json::to_string(&resolved.0).map_err(Into::into)
    } else {
        serde_json::to_string_pretty(&resolved.0).map_err(Into::into)
    }
}

pub fn write_output(
    output: &str,
    output_file: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(output_file) = output_file {
        fs::write(output_file, output).map_err(Into::into)
    } else {
        println!("{output}");
        Ok(())
    }
}
