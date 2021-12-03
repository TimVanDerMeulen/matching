extern crate regex;
extern crate serde;
extern crate serde_json;

use connections::{Connection, Connector};
use rules::{Rule, RuleActions, RuleSeverity};
use score::Scorer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

mod connections;
mod rules;
mod score;

#[derive(Serialize, Deserialize)]
struct MatchingData {
    fields: HashMap<String, String>, // id -> field name/text
    elements: HashMap<String, HashMap<String, String>>, // id -> { fieldId -> value }
    rules: Vec<Rule>,
}

fn prepare_connections(
    elements: &HashMap<String, HashMap<String, String>>,
) -> HashMap<String, Vec<Connection>> {
    let mut connections: HashMap<String, Vec<Connection>> = HashMap::new();
    let mut element_iter = elements.keys().into_iter();
    while let Some(id) = element_iter.next() {
        connections.insert(id.to_string(), Vec::new());
    }
    return connections;
}

fn add_connections(
    elements: &HashMap<String, HashMap<String, String>>,
    connections: &mut HashMap<String, Vec<Connection>>,
    rules: &Vec<Rule>,
) {
    if rules.is_empty() {
        return;
    }
    connections.connect(|id, all| {
        for rule in rules {
            // add connection to all elements that match the rule
            let mut all_elements_iter = elements.iter().map(|(e_id, _)| e_id).filter(|e| e != &id);
            for target_id in all_elements_iter {
                if rule.check(id, target_id, elements) {
                    rule.apply(id, target_id, all);
                }
            }
        }
    });
}

fn main() {
    let data = fs::read_to_string("res/concept.json").expect("Unable to read file");
    let matching_data: MatchingData =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    let mut connections = prepare_connections(&matching_data.elements);

    // print keys
    println!("----------------------------------------------------------------");
    println!("elements:");
    connections.connect(|id, all| println!("{}", id));

    for severity in [
        RuleSeverity::Prefer,
        RuleSeverity::Standard,
        RuleSeverity::PreferExclude,
    ] {
        add_connections(
            &matching_data.elements,
            &mut connections,
            &matching_data
                .rules
                .iter()
                .filter(|rule| rule.severity == severity)
                .cloned()
                .collect(),
        );
    }
    add_connections(
        &matching_data.elements,
        &mut connections,
        &matching_data
            .rules
            .iter()
            .filter(|rule| rule.severity == RuleSeverity::ForceExclude)
            .cloned()
            .collect(),
    );
    add_connections(
        &matching_data.elements,
        &mut connections,
        &matching_data
            .rules
            .iter()
            .filter(|rule| rule.severity == RuleSeverity::Force)
            .cloned()
            .collect(),
    );

    // print connections
    println!("----------------------------------------------------------------");
    println!("connections:");
    connections.connect(|id, all| {
        let mut cons = all.get_mut(id).expect("Id not found!");
        for con in cons {
            println!("{} -> {} (score: {})", id, con.to_element, con.score)
        }
    });
}
