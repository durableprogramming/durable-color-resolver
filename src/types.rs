use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorDefinitions(pub HashMap<String, String>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedColors(pub HashMap<String, String>);
