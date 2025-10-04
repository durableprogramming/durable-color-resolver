use crate::parser::expressions::parse_expression_with_visited;

pub fn parse_args(
    args_str: &str,
    color_map: &std::collections::HashMap<String, String>,
) -> Result<Vec<String>, String> {
    parse_args_with_visited(args_str, color_map, &mut std::collections::HashSet::new())
}

pub fn parse_args_with_visited(
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
