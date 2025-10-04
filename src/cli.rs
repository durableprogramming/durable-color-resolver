use clap::{Arg, ArgAction, Command};

pub fn print_available_functions() {
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

pub fn build_cli() -> Command {
    Command::new("durable-color-resolver")
        .version("1.0")
        .author("Durable Programming, LLC")
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
}
