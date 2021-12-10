extern crate serde;
extern crate serde_json;

use crate::matching::rules::Rule;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct MatchingData {
    fields: HashMap<String, String>, // id -> field name/text
    pub(crate) elements: HashMap<String, HashMap<String, String>>, // id -> { fieldId -> value }
    pub(crate) rules: Vec<Rule>,
    pub(crate) outputs: HashMap<usize, i16>, // possible output sizes: size -> max usages
}

#[derive(Serialize, Deserialize)]
pub(crate) struct MatchingResult {
    pub score: i32,
    pub connections: Vec<Vec<usize>>,
}
