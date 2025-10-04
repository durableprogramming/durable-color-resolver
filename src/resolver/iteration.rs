use crate::ColorDefinitions;
use std::collections::HashMap;
use std::collections::HashSet;

use super::validation;

pub fn run_resolution_iterations(
    definitions: &ColorDefinitions,
    resolved: &mut HashMap<String, String>,
    unresolved: &mut HashSet<String>,
) {
    let mut changed = true;
    while changed && !unresolved.is_empty() {
        changed = false;
        let keys: Vec<String> = unresolved.iter().cloned().collect();
        // Create merged map: definitions + resolved (resolved takes precedence)
        let mut merged: HashMap<String, String> = definitions.0.clone();
        for (k, v) in &*resolved {
            merged.insert(k.clone(), v.clone());
        }
        for key in keys {
            if let Some(value) = definitions.0.get(&key) {
                match crate::parser::parse_expression(value, &merged) {
                    Ok(result) => {
                        if validation::is_valid_color_value(&result, definitions) {
                            resolved.insert(key.clone(), result);
                            unresolved.remove(&key);
                            changed = true;
                        }
                    }
                    Err(_) => {
                        // Dependency not resolved yet
                    }
                }
            }
        }
    }
}
