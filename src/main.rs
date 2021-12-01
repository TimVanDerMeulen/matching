extern crate serde_json;
extern crate serde;

use std::fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod rules;

#[derive(Serialize, Deserialize)]
struct MatchingData {
    fields: HashMap<String, String>, // id -> field name/text
    elements: HashMap<String, HashMap<String, String>>, // id -> { fieldId -> value }
    rules: rules::Rules,
}

fn add_possible_connections(){

}

fn main() {
    let data = fs::read_to_string("res/concept.json").expect("Unable to read file");
    let json: serde_json::Value =
        serde_json::from_str(&data).expect("JSON was not well-formatted");
    println!("{}", json)
}
