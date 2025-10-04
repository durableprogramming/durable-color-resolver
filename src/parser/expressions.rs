use super::args::parse_args_with_visited;
use super::color_resolution::resolve_color_value;
use super::primitives::{func_call, parse_pipe};
use crate::constants;
use crate::functions::*;

pub fn parse_expression(
    expr: &str,
    color_map: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    parse_expression_with_visited(expr, color_map, &mut std::collections::HashSet::new())
}

pub fn parse_expression_with_visited(
    expr: &str,
    color_map: &std::collections::HashMap<String, String>,
    visited: &mut std::collections::HashSet<String>,
) -> Result<String, String> {
    let expr = expr.trim();

    // Check if it contains pipe operators
    if let Ok((remaining, parts_raw)) = parse_pipe(expr) {
        if remaining.is_empty() {
            let parts: Vec<&str> = parts_raw.iter().map(|s| s.trim()).collect();
            if parts.is_empty() {
                return Ok("".to_string());
            }

            // Start with the first part
            let mut result = parse_single_expression_with_visited(parts[0], color_map, visited)?;

            // Apply each piped function
            for part in &parts[1..] {
                // Each part after | should be a function call like "darken(0.2)"
                if let Ok((remaining, (func_name, args_str))) = func_call(part) {
                    if remaining.is_empty() {
                        let mut args = parse_args_with_visited(args_str, color_map, visited)?;
                        // Prepend the current result as the first argument
                        args.insert(0, result.clone());

                        result = match func_name {
                                "darken" => {
                                    if args.len() != 2 {
                                        return Err(format!(
                                            "Function 'darken' requires exactly 2 arguments (color, amount), got {}. \
                                             Example: darken(blue-500, 20%)",
                                            args.len()
                                        ));
                                    }
                                     darken(&args[0], &args[1])?
                                }
                                "lighten" => {
                                    if args.len() != 2 {
                                        return Err(format!(
                                            "Function 'lighten' requires exactly 2 arguments (color, amount), got {}. \
                                             Example: lighten(blue-500, 15%)",
                                            args.len()
                                        ));
                                    }
                                    lighten(&args[0], &args[1])?
                                }
                                "temperature_up" => {
                                    if args.len() != 2 {
                                        return Err(format!(
                                            "Function 'temperature_up' requires exactly 2 arguments (color, amount), got {}. \
                                             Example: temperature_up(blue-500, 15)",
                                            args.len()
                                        ));
                                    }
                                    temperature_up(&args[0], &args[1])?
                                }
                                "temperature_down" => {
                                    if args.len() != 2 {
                                        return Err(format!(
                                            "Function 'temperature_down' requires exactly 2 arguments (color, amount), got {}. \
                                             Example: temperature_down(red-500, 20)",
                                            args.len()
                                        ));
                                    }
                                    temperature_down(&args[0], &args[1])?
                                }
                                "saturate" => {
                                    if args.len() != 2 {
                                        return Err(format!(
                                            "Function 'saturate' requires exactly 2 arguments (color, amount), got {}. \
                                             Example: saturate(gray, 0.2)",
                                            args.len()
                                        ));
                                    }
                                    saturate(&args[0], &args[1])?
                                }
                                "desaturate" => {
                                    if args.len() != 2 {
                                        return Err(format!(
                                            "Function 'desaturate' requires exactly 2 arguments (color, amount), got {}. \
                                             Example: desaturate(red-500, 0.3)",
                                            args.len()
                                        ));
                                    }
                                    desaturate(&args[0], &args[1])?
                                }
                                "mix" => {
                                    if args.len() != 3 {
                                        return Err(format!(
                                            "Function 'mix' requires exactly 3 arguments (color1, color2, amount), got {}. \
                                             Example: mix(red-500, blue-500, 0.3)",
                                            args.len()
                                        ));
                                    }
                                    mix(&args[0], &args[1], &args[2])?
                                }
                               _ => return Err(format!("Unknown function: '{func_name}'. \
                                                Available functions: darken, lighten, temperature_up, temperature_down, saturate, desaturate, mix. \
                                                Use --list-functions for detailed help.")),
                          };
                    } else {
                        return Err(format!(
                            "Invalid pipe syntax: '{part}' is not a valid function call. \
                                          Pipe syntax should be: color | function(args). \
                                          Example: blue-500 | darken(20%)"
                        ));
                    }
                } else {
                    return Err(format!(
                        "Invalid pipe syntax: '{part}' is not a valid function call. \
                                      Pipe syntax should be: color | function(args). \
                                      Example: blue-500 | darken(20%)"
                    ));
                }
            }

            Ok(result)
        } else {
            parse_single_expression_with_visited(expr, color_map, visited)
        }
    } else {
        parse_single_expression_with_visited(expr, color_map, visited)
    }
}

fn parse_single_expression_with_visited(
    expr: &str,
    color_map: &std::collections::HashMap<String, String>,
    visited: &mut std::collections::HashSet<String>,
) -> Result<String, String> {
    let expr = expr.trim();

    let _parse_other = || {
        // Check if it's a reference (case-insensitive)
        let expr_lower = expr.to_lowercase();
        if let Some(color) = color_map.get(expr).or_else(|| {
            color_map
                .keys()
                .find(|k| k.to_lowercase() == expr_lower)
                .and_then(|k| color_map.get(k))
        }) {
            // Recursively resolve the reference
            if visited.contains(expr) {
                // Circular reference, return the key
                Ok(expr.to_string())
            } else {
                visited.insert(expr.to_string());
                let result = parse_expression_with_visited(color, color_map, visited);
                visited.remove(expr);
                result
            }
        } else {
            // Check if it's a constant (case-insensitive)
            let constants = constants::color_constants();
            if let Some(color) = constants
                .get(expr)
                .or_else(|| constants.get(&expr.to_lowercase()))
            {
                resolve_color_value(color)
            } else {
                // Check if it's a valid color literal
                if parse_color(expr).is_ok() || expr.starts_with('#') {
                    Ok(expr.to_string())
                } else {
                    // Allow literal parameters (like "20%", "0.2", "30") that are not color references
                    Ok(expr.to_string())
                }
            }
        }
    };

    // Check if it's an OKLCH color literal
    if expr.starts_with("oklch(") {
        return resolve_color_value(expr);
    }

    // Check if it's a function call
    if let Ok((remaining, (func_name, args_str))) = func_call(expr) {
        if remaining.is_empty() {
            let args = parse_args_with_visited(args_str, color_map, visited)?;
            match func_name {
                "darken" | "lighten" | "temperature_up" | "temperature_down" | "saturate"
                | "desaturate" => {
                    if args.len() != 2 {
                        Err(format!(
                            "Function '{}' requires exactly 2 arguments (color, amount), got {}. \
                              Use --list-functions for examples.",
                            func_name,
                            args.len()
                        ))
                    } else {
                        match func_name {
                            "darken" => darken(&args[0], &args[1]),
                            "lighten" => lighten(&args[0], &args[1]),
                            "temperature_up" => temperature_up(&args[0], &args[1]),
                            "temperature_down" => temperature_down(&args[0], &args[1]),
                            "saturate" => saturate(&args[0], &args[1]),
                            "desaturate" => desaturate(&args[0], &args[1]),
                            _ => unreachable!(),
                        }
                    }
                }
                "mix" => {
                    if args.len() != 3 {
                        Err(format!(
                            "Function 'mix' requires exactly 3 arguments (color1, color2, amount), got {}. \
                              Example: mix(red-500, blue-500, 0.3)",
                            args.len()
                        ))
                    } else {
                        mix(&args[0], &args[1], &args[2])
                    }
                }
                _ => Err(format!("Unknown function: '{func_name}'. \
                          Available functions: darken, lighten, temperature_up, temperature_down, saturate, desaturate, mix. \
                          Use --list-functions for detailed help.")),
            }
        } else {
            Err(format!(
                "Invalid function call: '{expr}'. \
                      Function calls should be: function_name(arg1, arg2, ...). \
                      Example: darken(blue-500, 20%)"
            ))
        }
    } else {
        // Check if it's a reference (case-insensitive)
        let expr_lower = expr.to_lowercase();
        if let Some(color) = color_map.get(expr).or_else(|| {
            color_map
                .keys()
                .find(|k| k.to_lowercase() == expr_lower)
                .and_then(|k| color_map.get(k))
        }) {
            // Recursively resolve the reference
            if visited.contains(expr) {
                // Circular reference, return the key
                Ok(expr.to_string())
            } else {
                visited.insert(expr.to_string());
                let result = parse_expression_with_visited(color, color_map, visited);
                visited.remove(expr);
                result
            }
        } else {
            // Check if it's a reference to a defined but unresolved key
            // This will be handled by returning an error to defer resolution
            // We don't check this here, as the resolver handles it
            // Check if it's a constant (case-insensitive)
            let constants = constants::color_constants();
            if let Some(color) = constants
                .get(expr)
                .or_else(|| constants.get(&expr.to_lowercase()))
            {
                resolve_color_value(color)
            } else {
                // Check if it's a valid color literal
                if parse_color(expr).is_ok() || expr.starts_with('#') {
                    Ok(expr.to_string())
                } else {
                    // Allow literal parameters (like "20%", "0.2", "30") that are not color references
                    Ok(expr.to_string())
                }
            }
        }
    }
}
