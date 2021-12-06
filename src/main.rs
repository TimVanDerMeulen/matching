extern crate regex;
extern crate serde;
extern crate serde_json;

use connections::{Connections, Connector};
use rules::{Rule, RuleActions};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

mod connections;
mod rules;
mod score;

#[derive(Serialize, Deserialize)]
struct MatchingData {
    fields: HashMap<String, String>, // id -> field name/text
    elements: HashMap<String, HashMap<String, String>>, // id -> { fieldId -> value }
    rules: Vec<Rule>,
    outputs: HashMap<u16, u16>, // possible output sizes: size -> max usages
}

fn main() {
    let data = fs::read_to_string("res/concept.json").expect("Unable to read file");
    let matching_data: MatchingData =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    let mut connections = Connections::from_data(&matching_data.elements);
    println!("{}", connections.to_string());
    for rule in matching_data.rules {
        println!("{}", rule.to_string());
        connections.apply(&rule, &matching_data.elements);
        println!("{}", connections.to_string());
    }

    println!("final result:");
    println!("{}", connections.to_string());
}
