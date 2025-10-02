use clap::{Arg, ArgAction, Command};
use durable_color_resolver::{resolve_colors, ColorDefinitions};
use std::fs;
use std::io::{self, Read};
use std::process;

fn print_available_functions() {
    println!("DURABLE COLOR RESOLVER - Available Functions & Syntax");
    println!("====================================================");
    println!();
    println!("COLOR FUNCTIONS:");
    println!("  darken(color, amount)        - Darken a color by the specified percentage");
    println!("  lighten(color, amount)       - Lighten a color by the specified percentage");
    println!("  temperature_up(color, amount) - Increase the color temperature (make warmer)");
    println!("  temperature_down(color, amount) - Decrease the color temperature (make cooler)");
    println!("  saturate(color, amount)      - Increase color saturation");
    println!("  desaturate(color, amount)    - Decrease color saturation");
    println!("  mix(color1, color2, amount)  - Mix two colors (amount is 0.0-1.0)");
    println!();
    println!("FUNCTION SYNTAX:");
    println!("  Traditional: darken(color, amount)");
    println!("  Pipe syntax:  color | darken(amount)");
    println!("  Chained:      color | darken(amount) | lighten(amount)");
    println!("  Nested:       darken(lighten(color, 10%), 5%)");
    println!();
    println!("AMOUNT FORMATS:");
    println!("  - darken/lighten: percentage (e.g., '20%', '10%')");
    println!("  - saturate/desaturate: decimal (e.g., '0.2', '0.5')");
    println!("  - temperature_up/down: numeric value (e.g., '15', '30')");
    println!("  - mix: decimal between 0.0 and 1.0 (e.g., '0.3')");
    println!();
    println!("SUPPORTED COLOR FORMATS:");
    println!("  - CSS named colors: red, blue, green, black, white, gray, etc.");
    println!("  - Tailwind colors: red-50 through red-900, blue-50 through blue-900, etc.");
    println!("  - Hex colors: #rrggbb or #rgb format");
    println!("  - OKLCH colors: oklch(lightness chroma hue) format");
    println!("  - Color references: Use defined color names as variables");
    println!();
    println!("EXAMPLES:");
    println!("  Basic usage:");
    println!("    {{\"primary\": \"blue-500\", \"secondary\": \"darken(primary, 20%)\"}}");
    println!();
    println!("  Pipe syntax:");
    println!("    {{\"accent\": \"red-500 | lighten(15%) | saturate(0.1)\"}}");
    println!();
    println!("  Temperature adjustment:");
    println!("    {{\"warm\": \"blue-500 | temperature_up(25)\", \"cool\": \"red-500 | temperature_down(20)\"}}");
    println!();
    println!("  Complex chaining:");
    println!(
        "    {{\"complex\": \"#ff0000 | darken(10%) | temperature_up(15) | desaturate(0.2)\"}}"
    );
    println!();
    println!("  Color mixing:");
    println!("    {{\"mixed\": \"mix(red-500, blue-500, 0.3)\"}}");
}

fn main() {
    let matches = Command::new("durable-color-resolver")
        .version("1.0")
        .author("Your Name")
        .about("Resolves color references using a domain-specific language (DSL)")
        .long_about("A powerful tool for resolving color references using a DSL based on CSS and Tailwind color constants. \
                      Supports color transformations and references, with all color math performed in the OKLCH color space.\n\n\
                      EXAMPLES:\n\
                      \n  Evaluate expression:\n\
                      \tdurable-color-resolver blue-500\n\
                      \tdurable-color-resolver 'darken(red, 20%)'\n\
                      \tdurable-color-resolver 'red | lighten(10%) | saturate(0.2)'\n\
                      \n  Read from file:\n\
                      \tdurable-color-resolver colors.json\n\
                      \n  Read from stdin:\n\
                      \tcat colors.json | durable-color-resolver -\n\
                      \n  Output to file:\n\
                      \tdurable-color-resolver colors.json -o resolved.json\n\
                      \tdurable-color-resolver 'darken(red, 20%)' -o result.txt\n\
                      \n  Compact output:\n\
                      \tdurable-color-resolver colors.json -c\n\
                      \n  Verbose mode:\n\
                      \tdurable-color-resolver colors.json -v\n\
                      \n  List functions:\n\
                      \tdurable-color-resolver --list-functions\n\
                      \nINPUT FORMATS:\n\
                      \n  Expression mode (no file):\n\
                      \tDirect color expressions: blue-500, #ff0000, oklch(0.5 0.1 120)\n\
                      \tFunction calls: darken(color, amount), lighten(color, amount)\n\
                      \tPipe syntax: color | function(args) | function(args)\n\
                      \n  File mode:\n\
                      \tJSON with string keys and string values representing color expressions.\n\
                      \t{\n\t  \"primary\": \"blue-500\",\n\t  \"secondary\": \"darken(primary, 20%)\"\n\t}\n\
                      \nUse --list-functions to see all available color functions and syntax.")
         .arg(
             Arg::new("input")
                 .help("Input JSON file containing color definitions, or - for stdin")
                 .long_help("Path to a JSON file with color definitions, or use '-' to read from standard input. \
                            The JSON should have string keys and string values representing color expressions.")
                 .index(1)
                 .required(false),
         )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Write output to FILE instead of stdout")
                .long_help("Specify an output file path. If omitted, results are written to standard output.")
                .value_name("FILE"),
        )
        .arg(
            Arg::new("compact")
                .short('c')
                .long("compact")
                .help("Output compact JSON instead of pretty-printed")
                .long_help("Produce minified JSON output without extra whitespace. \
                           Useful for production builds or when piping to other tools.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Show detailed progress information")
                .long_help("Enable verbose output showing parsing progress, resolution statistics, \
                           and file operations. Useful for debugging complex color definitions.")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("list-functions")
                .long("list-functions")
                .help("Display available color functions and exit")
                .long_help("Print a comprehensive list of all supported color functions, \
                           their syntax, and usage examples, then exit without processing any input.")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Handle --list-functions option
    if matches.get_flag("list-functions") {
        print_available_functions();
        return;
    }

    let input = matches.get_one::<String>("input");

    let (is_expression_mode, input_source) = if let Some(input) = input {
        // Check if it's a file that exists or "-"
        if input == "-" || std::path::Path::new(input).exists() {
            (false, input.clone())
        } else {
            // Treat as expression
            (true, input.clone())
        }
    } else {
        eprintln!("Error: Input file or expression is required unless --list-functions is used.");
        eprintln!("Use --help for usage information.");
        process::exit(1);
    };
    let verbose = matches.get_flag("verbose");
    let compact = matches.get_flag("compact");

    if verbose {
        if is_expression_mode {
            eprintln!("Evaluating expression: {}", input_source);
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
        use durable_color_resolver::parser;
        let color_map = std::collections::HashMap::new();
        match parser::parse_expression(&input_source, &color_map) {
            Ok(result) => {
                if let Some(output_file) = matches.get_one::<String>("output") {
                    if let Err(e) = std::fs::write(output_file, &result) {
                        eprintln!("Error writing to file '{}': {}", output_file, e);
                        eprintln!("Check that you have write permissions for the directory.");
                        process::exit(1);
                    }
                    if verbose {
                        eprintln!("Output written to: {}", output_file);
                    }
                } else {
                    println!("{}", result);
                }
                return;
            }
            Err(e) => {
                eprintln!("Error evaluating expression: {}", e);
                eprintln!("Use --list-functions to see valid syntax and examples.");
                process::exit(1);
            }
        }
    }

    let json_str = if &input_source == "-" {
        let mut buffer = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut buffer) {
            eprintln!("Error reading from stdin: {}", e);
            eprintln!("Make sure you're piping valid JSON data to the command.");
            eprintln!("Example: cat colors.json | durable-color-resolver -");
            process::exit(1);
        }
        buffer
    } else {
        match fs::read_to_string(&input_source) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading file '{}': {}", input_source, e);
                eprintln!("Check that the file exists and you have read permissions.");
                eprintln!("Use '-' as input to read from stdin instead.");
                process::exit(1);
            }
        }
    };

    // First, try to parse as a generic JSON value to provide better error messages
    let json_value: serde_json::Value = match serde_json::from_str(&json_str) {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error parsing JSON: {}", e);
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
            process::exit(1);
        }
    };

    // Validate that it's an object
    match &json_value {
        serde_json::Value::Object(obj) => {
            // Validate that all values are strings
            for (key, value) in obj {
                if !value.is_string() {
                    eprintln!("Error: All color values must be strings.");
                    eprintln!("Key '{}' has value '{}' which is not a string.", key, value);
                    eprintln!("Expected: \"{}\": \"color-expression\"", key);
                    eprintln!("Use --list-functions to see valid color expressions.");
                    process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Error: Input JSON must be an object (key-value pairs).");
            eprintln!("Expected format: {{\"colorName\": \"colorExpression\", ...}}");
            eprintln!("But received: {}", json_value);
            process::exit(1);
        }
    };

    // Now parse as ColorDefinitions
    let definitions: ColorDefinitions = match serde_json::from_str(&json_str) {
        Ok(defs) => defs,
        Err(e) => {
            eprintln!("Error converting JSON to color definitions: {}", e);
            eprintln!("This is likely an internal error. Please report this issue.");
            process::exit(1);
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

    let json_output = if compact {
        match serde_json::to_string(&resolved.0) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error serializing compact output: {}", e);
                eprintln!("This is likely an internal error. Please report this issue.");
                process::exit(1);
            }
        }
    } else {
        match serde_json::to_string_pretty(&resolved.0) {
            Ok(json) => json,
            Err(e) => {
                eprintln!("Error serializing pretty output: {}", e);
                eprintln!("This is likely an internal error. Please report this issue.");
                process::exit(1);
            }
        }
    };

    if let Some(output_file) = matches.get_one::<String>("output") {
        if let Err(e) = fs::write(output_file, &json_output) {
            eprintln!("Error writing to file '{}': {}", output_file, e);
            eprintln!("Check that you have write permissions for the directory.");
            eprintln!(
                "Try writing to a different location or omit the -o flag to output to stdout."
            );
            process::exit(1);
        }
        if verbose {
            eprintln!("Output written to: {}", output_file);
        }
    } else {
        println!("{}", json_output);
    }
}
