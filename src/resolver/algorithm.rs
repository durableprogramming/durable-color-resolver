use crate::ColorDefinitions;
use crate::ResolvedColors;
use std::collections::HashMap;
use std::collections::HashSet;

use super::error_handling;
use super::iteration;

pub fn resolve_core(definitions: ColorDefinitions) -> ResolvedColors {
    let mut resolved: HashMap<String, String> = HashMap::new();
    let mut unresolved: HashSet<String> = definitions.0.keys().cloned().collect();

    iteration::run_resolution_iterations(&definitions, &mut resolved, &mut unresolved);

    error_handling::finalize_unresolved(&mut resolved, &unresolved);

    ResolvedColors(resolved)
}
