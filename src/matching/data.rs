extern crate serde;
extern crate serde_json;

use crate::matching::rules::Rule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MatchingData {
    pub(crate) fields: HashMap<String, String>, // id -> field name/text
    pub(crate) elements: HashMap<String, HashMap<String, String>>, // id -> { fieldId -> value }
    pub(crate) rules: Vec<Rule>,
    pub(crate) outputs: HashMap<usize, i16>, // possible output sizes: size -> max usages
}

impl Default for MatchingData {
    fn default() -> Self {
        Self {
            fields: Default::default(),
            elements: Default::default(),
            rules: vec![],
            outputs: Default::default(),
        }
    }
}

impl MatchingData {
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            elements: HashMap::new(),
            rules: Vec::new(),
            outputs: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct MatchingResult {
    pub score: i32,
    pub connections: Vec<Vec<usize>>,
}
