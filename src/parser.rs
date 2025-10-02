use crate::constants;
use crate::functions;
use nom::{
    bytes::complete::take_while1,
    character::complete::{char, space0},
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

fn balanced_parens(input: &str) -> IResult<&str, &str> {
    let mut depth = 1; // start after the opening '('
    let mut end = 0;
    let mut chars = input.char_indices();
    while let Some((i, c)) = chars.next() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    end = i;
                    break;
                }
            }
            _ => {}
        }
    }
    if depth == 0 {
        Ok((&input[end + 1..], &input[..end]))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )))
    }
}

fn func_call(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = space0(input)?;
    let (input, func_name) = take_while1(|c: char| c.is_alphabetic() || c == '_')(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('(')(input)?;
    let (input, args_str) = balanced_parens(input)?;
    let (input, _) = space0(input)?;
    Ok((input, (func_name, args_str)))
}

fn parse_pipe(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(
        delimited(space0, char('|'), space0),
        take_while1(|c| c != '|'),
    )(input)
}

fn resolve_color_value(color_str: &str) -> Result<String, String> {
    if color_str.starts_with("oklch(") {
        functions::parse_color(color_str).map(functions::color_to_hex)
    } else {
        Ok(color_str.to_string())
    }
}

pub fn parse_expression(
    expr: &str,
    color_map: &std::collections::HashMap<String, String>,
) -> Result<String, String> {
    parse_expression_with_visited(expr, color_map, &mut std::collections::HashSet::new())
}

fn parse_expression_with_visited(
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
                                  functions::darken(&args[0], &args[1])?
                              }
                              "lighten" => {
                                  if args.len() != 2 {
                                      return Err(format!(
                                          "Function 'lighten' requires exactly 2 arguments (color, amount), got {}. \
                                           Example: lighten(blue-500, 15%)",
                                          args.len()
                                      ));
                                  }
                                  functions::lighten(&args[0], &args[1])?
                              }
                              "temperature_up" => {
                                  if args.len() != 2 {
                                      return Err(format!(
                                          "Function 'temperature_up' requires exactly 2 arguments (color, amount), got {}. \
                                           Example: temperature_up(blue-500, 15)",
                                          args.len()
                                      ));
                                  }
                                  functions::temperature_up(&args[0], &args[1])?
                              }
                              "temperature_down" => {
                                  if args.len() != 2 {
                                      return Err(format!(
                                          "Function 'temperature_down' requires exactly 2 arguments (color, amount), got {}. \
                                           Example: temperature_down(red-500, 20)",
                                          args.len()
                                      ));
                                  }
                                  functions::temperature_down(&args[0], &args[1])?
                              }
                              "saturate" => {
                                  if args.len() != 2 {
                                      return Err(format!(
                                          "Function 'saturate' requires exactly 2 arguments (color, amount), got {}. \
                                           Example: saturate(gray, 0.2)",
                                          args.len()
                                      ));
                                  }
                                  functions::saturate(&args[0], &args[1])?
                              }
                              "desaturate" => {
                                  if args.len() != 2 {
                                      return Err(format!(
                                          "Function 'desaturate' requires exactly 2 arguments (color, amount), got {}. \
                                           Example: desaturate(red-500, 0.3)",
                                          args.len()
                                      ));
                                  }
                                  functions::desaturate(&args[0], &args[1])?
                              }
                              "mix" => {
                                  if args.len() != 3 {
                                      return Err(format!(
                                          "Function 'mix' requires exactly 3 arguments (color1, color2, amount), got {}. \
                                           Example: mix(red-500, blue-500, 0.3)",
                                          args.len()
                                      ));
                                  }
                                  functions::mix(&args[0], &args[1], &args[2])?
                              }
                              _ => return Err(format!("Unknown function: '{func_name}'. \
                                               Available functions: darken, lighten, temperature_up, temperature_down, saturate, desaturate, mix. \
                                               Use --list-functions for detailed help.")),
                         };
                    } else {
                        return Err(format!(
                            "Invalid pipe syntax: '{}' is not a valid function call. \
                                         Pipe syntax should be: color | function(args). \
                                         Example: blue-500 | darken(20%)",
                            part
                        ));
                    }
                } else {
                    return Err(format!(
                        "Invalid pipe syntax: '{}' is not a valid function call. \
                                     Pipe syntax should be: color | function(args). \
                                     Example: blue-500 | darken(20%)",
                        part
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

    let parse_other = || {
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
                if functions::parse_color(expr).is_ok() || expr.starts_with('#') {
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
                            "darken" => functions::darken(&args[0], &args[1]),
                            "lighten" => functions::lighten(&args[0], &args[1]),
                            "temperature_up" => functions::temperature_up(&args[0], &args[1]),
                            "temperature_down" => functions::temperature_down(&args[0], &args[1]),
                            "saturate" => functions::saturate(&args[0], &args[1]),
                            "desaturate" => functions::desaturate(&args[0], &args[1]),
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
                        functions::mix(&args[0], &args[1], &args[2])
                    }
                }
                _ => Err(format!("Unknown function: '{func_name}'. \
                         Available functions: darken, lighten, temperature_up, temperature_down, saturate, desaturate, mix. \
                         Use --list-functions for detailed help.")),
            }
        } else {
            Err(format!(
                "Invalid function call: '{}'. \
                     Function calls should be: function_name(arg1, arg2, ...). \
                     Example: darken(blue-500, 20%)",
                expr
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
                if functions::parse_color(expr).is_ok() || expr.starts_with('#') {
                    Ok(expr.to_string())
                } else {
                    // Allow literal parameters (like "20%", "0.2", "30") that are not color references
                    Ok(expr.to_string())
                }
            }
        }
    }
}

fn parse_args(
    args_str: &str,
    color_map: &std::collections::HashMap<String, String>,
) -> Result<Vec<String>, String> {
    parse_args_with_visited(args_str, color_map, &mut std::collections::HashSet::new())
}

fn parse_args_with_visited(
    args_str: &str,
    color_map: &std::collections::HashMap<String, String>,
    visited: &mut std::collections::HashSet<String>,
) -> Result<Vec<String>, String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut escaped = false;

    for ch in args_str.chars() {
        match ch {
            '(' if !in_string => {
                depth += 1;
                current.push(ch);
            }
            ')' if !in_string => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 && !in_string => {
                if !current.trim().is_empty() {
                    args.push(parse_expression_with_visited(
                        current.trim(),
                        color_map,
                        visited,
                    )?);
                }
                current.clear();
            }
            '"' if !escaped => {
                in_string = !in_string;
                // Don't push quotes to current
            }
            '\\' if in_string => {
                escaped = true;
                current.push(ch);
            }
            _ => {
                escaped = false;
                current.push(ch);
            }
        }
    }

    if !current.trim().is_empty() {
        args.push(parse_expression_with_visited(
            current.trim(),
            color_map,
            visited,
        )?);
    }

    Ok(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_parse_simple_expression() {
        let color_map = HashMap::new();
        let result = parse_expression("blue-500", &color_map);
        assert!(result.unwrap().starts_with("#")); // Constants get resolved to hex
    }

    #[test]
    fn test_parse_hex_literal() {
        let color_map = HashMap::new();
        let result = parse_expression("#ff0000", &color_map);
        assert_eq!(result.unwrap(), "#ff0000");
    }

    #[test]
    fn test_parse_reference() {
        let mut color_map = HashMap::new();
        color_map.insert("primary".to_string(), "#ff0000".to_string());
        let result = parse_expression("primary", &color_map);
        assert_eq!(result.unwrap(), "#ff0000");
    }

    #[test]
    fn test_parse_function_call() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("darken(base, 20%)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_nested_function_calls() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("lighten(darken(base, 10%), 5%)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_args_simple() {
        let color_map = HashMap::new();
        let result = parse_args("arg1, arg2", &color_map);
        assert_eq!(result.unwrap(), vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_parse_args_with_functions() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_args("base, 20%", &color_map);
        assert_eq!(result.unwrap(), vec!["#ff0000", "20%"]);
    }

    #[test]
    fn test_parse_args_nested() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_args("darken(base, 10%), 5%", &color_map).unwrap();
        assert_eq!(result.len(), 2);
        assert!(result[0].starts_with("#"));
        assert_eq!(result[1], "5%");
    }

    #[test]
    fn test_parse_args_with_parentheses() {
        let color_map = HashMap::new();
        let result = parse_args("darken(#ff0000, 10%), lighten(#00ff00, 5%)", &color_map);
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
        assert!(args[0].starts_with("#"));
        assert!(args[1].starts_with("#"));
    }

    #[test]
    fn test_parse_unknown_function() {
        let color_map = HashMap::new();
        let result = parse_expression("unknown_func(color, 10%)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_parse_invalid_color() {
        let color_map = HashMap::new();
        let result = parse_expression("invalid(color)", &color_map);
        assert!(result.is_err());
    }

    #[test]
    fn test_resolve_color_value_oklch() {
        // This is a private function, but we can test it indirectly
        let mut color_map = HashMap::new();
        color_map.insert("oklch_color".to_string(), "oklch(0.5 0.1 120)".to_string());
        let result = parse_expression("oklch_color", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_expression_with_spaces() {
        let color_map = HashMap::new();
        let result = parse_expression("  blue-500  ", &color_map);
        assert!(result.unwrap().starts_with("#")); // Constants get resolved to hex
    }

    #[test]
    fn test_parse_complex_expression() {
        let mut color_map = HashMap::new();
        color_map.insert("primary".to_string(), "blue-500".to_string());
        color_map.insert("secondary".to_string(), "red-500".to_string());
        let result = parse_expression(
            "darken(lighten(primary, 10%), temperature_up(secondary, 15))",
            &color_map,
        );
        // This should fail because temperature_up expects a number, not a color
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_temperature_function() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("temperature_up(base, 30)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_saturate_function() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#808080".to_string());
        let result = parse_expression("saturate(base, 0.2)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_desaturate_function() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("desaturate(base, 0.5)", &color_map);
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_chained_references() {
        let mut color_map = HashMap::new();
        color_map.insert("level1".to_string(), "blue-500".to_string());
        color_map.insert("level2".to_string(), "level1".to_string());
        let result = parse_expression("level2", &color_map);
        assert!(result.unwrap().starts_with('#')); // Resolves transitively
    }

    #[test]
    fn test_parse_empty_args() {
        let color_map = HashMap::new();
        let result = parse_args("", &color_map);
        assert_eq!(result.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn test_parse_single_arg() {
        let color_map = HashMap::new();
        let result = parse_args("single", &color_map);
        assert_eq!(result.unwrap(), vec!["single"]);
    }

    #[test]
    fn test_parse_args_with_commas_in_functions() {
        let color_map = HashMap::new();
        // Test that commas inside function calls are handled
        let result = parse_args("darken(#ff0000, 10%), #00ff00", &color_map);
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
        assert!(args[0].starts_with("#"));
        assert_eq!(args[1], "#00ff00");
    }

    #[test]
    fn test_parse_malformed_oklch() {
        let color_map = HashMap::new();
        let result = parse_expression("oklch(invalid)", &color_map);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_function_args() {
        let color_map = HashMap::new();
        let result = parse_expression("darken()", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires exactly 2 arguments"));
    }

    #[test]
    fn test_parse_function_with_too_many_args() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("darken(base, 10%, extra)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires exactly 2 arguments"));
    }

    #[test]
    fn test_parse_function_with_too_few_args() {
        let color_map = HashMap::new();
        let result = parse_expression("darken(#ff0000)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("requires exactly 2 arguments"));
    }

    #[test]
    fn test_parse_function_with_spaces_in_args() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("darken( base , 10% )", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_nested_function_with_spaces() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("lighten( darken( base , 10% ) , 5% )", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_mixed_references_and_literals() {
        let mut color_map = HashMap::new();
        color_map.insert("primary".to_string(), "#ff0000".to_string());
        let result = parse_expression("darken(primary, 20%)", &color_map);
        assert!(result.is_ok());
        let result2 = parse_expression("darken(#00ff00, 20%)", &color_map);
        assert!(result2.is_ok());
        assert_ne!(result.unwrap(), result2.unwrap());
    }

    #[test]
    fn test_parse_invalid_function_name() {
        let color_map = HashMap::new();
        let result = parse_expression("invalid_func(#ff0000, 10%)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_parse_function_case_sensitivity() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("DARKEN(base, 10%)", &color_map);
        assert!(result.is_err()); // Functions are case-sensitive
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_parse_color_reference_case_sensitivity() {
        let mut color_map = HashMap::new();
        color_map.insert("primary".to_string(), "#ff0000".to_string());
        let result = parse_expression("PRIMARY", &color_map);
        assert_eq!(result.unwrap(), "#ff0000"); // References are case-sensitive
    }

    #[test]
    fn test_parse_constant_case_insensitive() {
        let color_map = HashMap::new();
        let result = parse_expression("RED", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_hex_case_insensitive() {
        let color_map = HashMap::new();
        let result = parse_expression("#FF0000", &color_map);
        assert_eq!(result.unwrap(), "#FF0000");
    }

    #[test]
    fn test_parse_empty_expression() {
        let color_map = HashMap::new();
        let result = parse_expression("", &color_map);
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_parse_whitespace_only() {
        let color_map = HashMap::new();
        let result = parse_expression("   ", &color_map);
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_parse_args_with_quotes() {
        let color_map = HashMap::new();
        let result = parse_args("\"arg1\", \"arg2\"", &color_map);
        assert_eq!(result.unwrap(), vec!["arg1", "arg2"]);
    }

    #[test]
    fn test_parse_args_with_escaped_quotes() {
        let color_map = HashMap::new();
        let result = parse_args("\"arg\\\"with\\\"quotes\", normal", &color_map);
        // This might not be fully implemented, but let's test current behavior
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
    }

    #[test]
    fn test_parse_complex_nested_args() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_args(
            "darken(base, 10%), lighten(darken(base, 5%), 15%)",
            &color_map,
        );
        let args = result.unwrap();
        assert_eq!(args.len(), 2);
        assert!(args[0].starts_with("#"));
        assert!(args[1].starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_simple() {
        let mut color_map = HashMap::new();
        color_map.insert("base".to_string(), "#ff0000".to_string());
        let result = parse_expression("base | darken(0.2)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_with_literal() {
        let color_map = HashMap::new();
        let result = parse_expression("red | darken(0.2)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_chained() {
        let color_map = HashMap::new();
        let result = parse_expression("red | darken(0.2) | lighten(0.1)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_with_hex() {
        let color_map = HashMap::new();
        let result = parse_expression("#ff0000 | darken(0.2)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }

    #[test]
    fn test_parse_pipe_invalid_function() {
        let color_map = HashMap::new();
        let result = parse_expression("red | invalid_func(0.2)", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown function"));
    }

    #[test]
    fn test_parse_pipe_invalid_syntax() {
        let color_map = HashMap::new();
        let result = parse_expression("red | not_a_function", &color_map);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid pipe syntax"));
    }

    #[test]
    fn test_parse_pipe_equivalent_to_function_call() {
        let color_map = HashMap::new();
        let pipe_result = parse_expression("red | darken(0.2)", &color_map);
        let func_result = parse_expression("darken(red, 0.2)", &color_map);
        assert_eq!(pipe_result.unwrap(), func_result.unwrap());
    }

    #[test]
    fn test_parse_pipe_with_spaces() {
        let color_map = HashMap::new();
        let result = parse_expression("red | darken(0.2) | lighten(0.1)", &color_map);
        assert!(result.is_ok());
        assert!(result.unwrap().starts_with("#"));
    }
}
