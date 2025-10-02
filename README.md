# Durable Color Resolver

A powerful tool for resolving color references using a domain-specific language (DSL) based on an expansive set of CSS and Tailwind color constants. It supports color transformations and references, with all color math performed in the OKLCH color space.

## Features

- **Color DSL**: Define colors using a simple expression language
- **Extensive Color Palette**: Includes CSS color names and Tailwind CSS color constants
- **Color Functions**: Supports transformations like `darken`, `lighten`, `temperature_up`, `temperature_down`, `saturate`, `desaturate`
- **Color References**: Colors can reference other defined colors
- **OKLCH Color Space**: All color calculations use the perceptually uniform OKLCH color space
- **Resolution Handling**: Unresolved color references are marked as `UNRESOLVED_COLOR` in the output
- **Rust Library and CLI**: Core functionality implemented in Rust with a command-line interface

## Installation

### CLI (Rust)

Download the latest binary from the [releases page](https://github.com/your-org/durable-color-resolver/releases) or build from source:

```bash
cargo install durable-color-resolver
```

### Library (Rust)

Add to your `Cargo.toml`:

```toml
[dependencies]
durable-color-resolver = "1.0"
```

## Usage

Define colors in a JSON structure using the DSL:

```json
{
  "primary": "blue-500",
  "secondary": "darken(primary, 20%)",
  "accent": "lighten(secondary, 10%)",
  "warm-accent": "temperature_up(accent, 15)",
  "cool-primary": "temperature_down(primary, 10)",
  "desaturated": "desaturate(primary, 0.3)",
  "saturated": "saturate(accent, 0.2)"
}
```

## CLI Usage

The CLI tool reads color definitions from a JSON file or stdin and outputs resolved colors to stdout or a file.

### Basic Usage

```bash
# From file
durable-color-resolver input.json

# From stdin
cat input.json | durable-color-resolver -

# Output to file
durable-color-resolver input.json -o output.json
```

### Options

- `input`: Input JSON file or `-` for stdin (required)
- `-o, --output <FILE>`: Output file (default: stdout)
- `-c, --compact`: Output compact JSON instead of pretty-printed
- `-v, --verbose`: Enable verbose output
- `--list-functions`: List available color functions and exit

### Examples

```bash
# Pretty-print output
durable-color-resolver colors.json

# Compact output to file
durable-color-resolver colors.json -o colors.min.json -c

# Verbose output
durable-color-resolver colors.json -v

# List available functions
durable-color-resolver --list-functions
```

## Color Functions

- `darken(color, amount)`: Darken a color by the specified percentage (e.g., `20%`)
- `lighten(color, amount)`: Lighten a color by the specified percentage (e.g., `10%`)
- `temperature_up(color, amount)`: Increase the color temperature (make warmer) by the specified amount (numeric value)
- `temperature_down(color, amount)`: Decrease the color temperature (make cooler) by the specified amount (numeric value)
- `saturate(color, amount)`: Increase color saturation (amount as decimal, e.g., `0.2`)
- `desaturate(color, amount)`: Decrease color saturation (amount as decimal, e.g., `0.5`)

### Function Syntax

Functions support both traditional and jq-like pipe syntax:

**Traditional syntax:**
```json
{
  "secondary": "darken(primary, 20%)",
  "accent": "lighten(darken(primary, 10%), 5%)"
}
```

**Pipe syntax:**
```json
{
  "secondary": "primary | darken(20%)",
  "accent": "primary | darken(10%) | lighten(5%)"
}
```

Both syntaxes are equivalent and can be mixed within the same definitions.

## Supported Color Formats

- **CSS Named Colors**: `red`, `blue`, `green`, `black`, `white`, `gray`, etc.
- **Tailwind Colors**: `red-50` through `red-900`, `blue-50` through `blue-900`, etc.
- **Hex Colors**: `#rrggbb` format
- **OKLCH Colors**: `oklch(lightness chroma hue)` format

## Rust API

```rust
use durable_color_resolver::{resolve_colors, ColorDefinitions};

let definitions = ColorDefinitions::from([
    ("primary".to_string(), "blue-500".to_string()),
    ("secondary".to_string(), "darken(primary, 20%)".to_string()),
]);

let resolved = resolve_colors(definitions);
println!("{:?}", resolved.0);
```

## Output Format

The tool outputs JSON with resolved color values as hex strings. Unresolved references are returned as:

```json
{
  "colorName": "UNRESOLVED_COLOR"
}
```

## Examples

### Input (Traditional Syntax)
```json
{
  "primary": "blue-500",
  "secondary": "darken(primary, 20%)",
  "accent": "lighten(secondary, 10%)",
  "warm": "temperature_up(accent, 15)",
  "cool": "temperature_down(primary, 10)"
}
```

### Input (Pipe Syntax)
```json
{
  "primary": "blue-500",
  "secondary": "primary | darken(20%)",
  "accent": "secondary | lighten(10%)",
  "warm": "accent | temperature_up(15)",
  "cool": "primary | temperature_down(10)"
}
```

### Output
```json
{
  "primary": "#3b82f6",
  "secondary": "#1e40af",
  "accent": "#3b75f6",
  "warm": "#4a7cf6",
  "cool": "#2d5ff6"
}
```

## Building from Source

### Rust
```bash
git clone https://github.com/your-org/durable-color-resolver.git
cd durable-color-resolver
cargo build --release
```

## Testing

### Rust
```bash
cargo test
```

## Contributing

Contributions are welcome! Please ensure all changes include comprehensive tests.

## License

MIT