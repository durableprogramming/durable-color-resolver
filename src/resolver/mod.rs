pub mod algorithm;
pub mod error_handling;
pub mod iteration;
pub mod validation;

#[cfg(test)]
mod tests;

use crate::ColorDefinitions;
use crate::ResolvedColors;

pub fn resolve_colors(definitions: ColorDefinitions) -> ResolvedColors {
    algorithm::resolve_core(definitions)
}
