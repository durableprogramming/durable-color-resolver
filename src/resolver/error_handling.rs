use std::collections::HashMap;
use std::collections::HashSet;

pub fn finalize_unresolved(resolved: &mut HashMap<String, String>, unresolved: &HashSet<String>) {
    for key in unresolved {
        resolved.insert(key.clone(), "UNRESOLVED_COLOR".to_string());
    }
}
