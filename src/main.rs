mod cli;
mod input;
mod output;

use durable_color_resolver::resolve_colors;

fn main() {
    let matches = cli::build_cli().get_matches();

    // Handle --list-functions option
    if matches.get_flag("list-functions") {
        cli::print_available_functions();
        return;
    }

    let input = matches.get_one::<String>("input");

    let (is_expression_mode, input_source) = input::determine_input_mode(input.map(|s| s.as_str()));

    if input.is_none() && !is_expression_mode {
        eprintln!("Error: Input file or expression is required unless --list-functions is used.");
        eprintln!("Use --help for usage information.");
        std::process::exit(1);
    }
    let verbose = matches.get_flag("verbose");
    let compact = matches.get_flag("compact");
    let output_file = matches.get_one::<String>("output").map(|s| s.as_str());

    if verbose {
        if is_expression_mode {
            eprintln!("Evaluating expression: {input_source}");
        } else {
            eprintln!(
                "Reading input from: {}",
                if input_source == "-" {
                    "stdin"
                } else {
                    &input_source
                }
            );
        }
    }

    // Handle expression mode
    if is_expression_mode {
        match input::parse_expression(&input_source) {
            Ok(result) => {
                if let Some(output_file) = output_file {
                    if let Err(e) = output::write_output(&result, Some(output_file)) {
                        eprintln!("Error writing to file '{output_file}': {e}");
                        eprintln!("Check that you have write permissions for the directory.");
                        std::process::exit(1);
                    }
                    if verbose {
                        eprintln!("Output written to: {output_file}");
                    }
                } else {
                    let _ = output::write_output(&result, None);
                }
                return;
            }
            Err(e) => {
                eprintln!("Error evaluating expression: {e}");
                eprintln!("Use --list-functions to see valid syntax and examples.");
                std::process::exit(1);
            }
        }
    }

    let json_str = match input::read_input(&input_source) {
        Ok(content) => content,
        Err(e) => {
            if input_source == "-" {
                eprintln!("Error reading from stdin: {e}");
                eprintln!("Make sure you're piping valid JSON data to the command.");
                eprintln!("Example: cat colors.json | durable-color-resolver -");
            } else {
                eprintln!("Error reading file '{input_source}': {e}");
                eprintln!("Check that the file exists and you have read permissions.");
                eprintln!("Use '-' as input to read from stdin instead.");
            }
            std::process::exit(1);
        }
    };

    let definitions = match input::parse_and_validate_json(&json_str) {
        Ok(defs) => defs,
        Err(e) => {
            eprintln!("Error parsing JSON: {e}");
            eprintln!("The input must be valid JSON.");
            eprintln!("Common issues:");
            eprintln!("  - Missing quotes around keys or values");
            eprintln!("  - Trailing commas");
            eprintln!("  - Unclosed brackets or braces");
            eprintln!("Example of valid JSON:");
            eprintln!("{{");
            eprintln!("  \"primary\": \"blue-500\",");
            eprintln!("  \"secondary\": \"darken(primary, 20%)\"");
            eprintln!("}}");
            std::process::exit(1);
        }
    };

    if verbose {
        eprintln!("Parsed {} color definitions", definitions.0.len());
    }

    let resolved = resolve_colors(definitions);

    if verbose {
        let unresolved_count = resolved
            .0
            .values()
            .filter(|v| *v == "UNRESOLVED_COLOR")
            .count();
        eprintln!(
            "Resolved {} colors, {} unresolved",
            resolved.0.len() - unresolved_count,
            unresolved_count
        );
    }

    let json_output = match output::format_output(&resolved, compact) {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Error serializing output: {e}");
            eprintln!("This is likely an internal error. Please report this issue.");
            std::process::exit(1);
        }
    };

    if let Err(e) = output::write_output(&json_output, output_file) {
        if let Some(output_file) = output_file {
            eprintln!("Error writing to file '{output_file}': {e}");
            eprintln!("Check that you have write permissions for the directory.");
            eprintln!(
                "Try writing to a different location or omit the -o flag to output to stdout."
            );
        } else {
            eprintln!("Error writing to stdout: {e}");
        }
        std::process::exit(1);
    }

    if verbose && output_file.is_some() {
        eprintln!("Output written to: {}", output_file.unwrap());
    }
}
