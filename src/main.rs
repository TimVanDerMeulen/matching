extern crate itertools;
extern crate regex;
extern crate serde;
extern crate serde_json;

use connections::{Connections, Connector};
use debug_print::debug_println;
use itertools::Itertools;
use rules::Rule;
use serde::{Deserialize, Serialize};
use std::cmp::max;
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
    outputs: HashMap<usize, i16>, // possible output sizes: size -> max usages
}

fn main() {
    let data = fs::read_to_string("res/concept.json").expect("Unable to read file");
    let matching_data: MatchingData =
        serde_json::from_str(&data).expect("JSON was not well-formatted");

    let mut connections = Connections::from_data(&matching_data.elements);
    //println!("{}", connections.to_string());
    for rule in matching_data.rules {
        //println!("{}", rule.to_string());
        connections.apply(&rule, &matching_data.elements);
        //println!("{}", connections.to_string());
    }

    debug_println!("preferences:");
    debug_println!("{}", connections.to_string());

    let mut possible_connections = connections.possible_connections();
    possible_connections.sort_by(|con1, con2| con1.len().cmp(&con2.len()));
    let connection_column_width = max(
        20,
        possible_connections[possible_connections.len() - 1].len(),
    );
    debug_println!(
        "{0: <5} | {1: <connection_column_width$} | {2: <connection_column_width$} | {3: <10}",
        "Index",
        "possible connections",
        "best combination",
        "best score",
        connection_column_width = connection_column_width
    );
    debug_println!("{}", "-".repeat(25 + 2 * connection_column_width));
    possible_connections.iter().for_each(|item| {
        //print!("{} -> {{ {}", item[0], item[0]);
        let mut combinations: Vec<(i32, Vec<&usize>)> = Vec::new();
        for output in &matching_data.outputs {
            let combinations_of_size = item.iter().skip(1).combinations(*output.0 - 1);
            for mut comb in combinations_of_size {
                comb.insert(0, &item[0]);
                let (possible, score) = connections.calc_score(&comb);
                if possible {
                    combinations.push((score, comb));
                }
            }
        }
        // sort by score
        combinations.sort_by(|comb1, comb2| comb2.0.cmp(&comb1.0));

        debug_println!(
            "{0: <5} | {1: >connection_column_width$} | {2: >connection_column_width$} | {3: >10}",
            item[0],
            format!("{:?}", item),
            format!("{:?}", combinations[0].1),
            combinations[0].0,
            connection_column_width = connection_column_width
        );
    });
}
