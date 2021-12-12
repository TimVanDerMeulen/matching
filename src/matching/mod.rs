extern crate itertools;
extern crate regex;
extern crate serde;
extern crate serde_json;

use self::itertools::Itertools;
use crate::matching::connections::{Connections, Connector};
use crate::matching::data::{MatchingData, MatchingResult};
use debug_print::debug_println;
use std::collections::{HashMap, HashSet};

pub mod connections;
pub mod data;
pub mod rules;
pub mod score;

fn calc_max_combinations(
    connections: &Connections,
    possible_connections: &Vec<Vec<usize>>,
    ignore: HashSet<usize>,
    outputs: &HashMap<usize, i16>, // (output size, available amount) | negative values are considered infinite
) -> (i32, Vec<Vec<usize>>) {
    if connections.len() == ignore.len() {
        return (0, Vec::new()); // all elements have been visited, so we need an empty list to return
    }
    let mut current_max: (i32, Vec<Vec<usize>>) = (i32::MIN, Vec::new());
    let item = &possible_connections[0];
    // debug_println!("{:?}", item);
    // debug_println!("    {:?}", possible_connections);
    // debug_println!("    {:?}", ignore);
    // debug_println!("    {:?}", outputs);
    if item.len() < 2 {
        return (i32::MIN, Vec::new());
    }
    if ignore.contains(&item[0]) {
        return calc_max_combinations(
            connections,
            &possible_connections
                .iter()
                .skip(1)
                .map(|elem| elem.clone())
                .collect(),
            ignore,
            outputs,
        );
    }
    let filtered_item: Vec<usize> = item
        .iter()
        .skip(1)
        .filter(|elem| !ignore.contains(elem))
        .map(|elem| *elem)
        .collect();
    if filtered_item.is_empty() {
        return (i32::MIN, Vec::new());
    }
    let mut combinations: Vec<(i32, Vec<&usize>)> = Vec::new();
    let max_output_size = outputs.keys().max().expect("No output available!");
    for output in 2..(*max_output_size + 1) {
        let combinations_of_size = filtered_item.iter().combinations(output - 1);
        // debug_println!("    {} -> {:?}", output - 1, combinations_of_size);
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
    // debug_println!("    {:?}", combinations);

    for (score, comb) in combinations {
        let mut output_size = comb.len();
        let mut output_opt = outputs.get(&output_size);
        if output_opt.is_none() {
            let mut keys: Vec<usize> = outputs
                .keys()
                .cloned()
                .filter(|k| k > &comb.len())
                .collect();
            keys.sort();
            output_size = keys[0];
            output_opt = outputs.get(&output_size);
        }
        let output = output_opt.expect("No viable output found!");

        let mut new_outputs = outputs.clone();
        match output {
            -1 => {} // keep because this output is infinite
            1 => {
                new_outputs.remove(&output_size);
            } // remove because this was the last available output of this size
            _ => {
                new_outputs.insert(output_size, output - 1);
            }
        }
        let mut new_ignore = ignore.clone();
        for elem in comb.clone() {
            new_ignore.insert(*elem);
        }
        let mut scenario = calc_max_combinations(
            connections,
            &possible_connections
                .iter()
                .skip(1)
                .map(|elem| elem.clone())
                .collect(),
            new_ignore,
            &new_outputs,
        );
        if scenario.0 == i32::MIN {
            continue;
        }
        scenario.0 += score;
        scenario.1.push(comb.iter().map(|elem| **elem).collect());
        if current_max.0 < scenario.0 {
            current_max = scenario
        }
    }
    return current_max;
}

pub fn process(matching_data: &MatchingData) -> MatchingResult {
    let mut connections = Connections::from_data(&matching_data.elements);
    //println!("{}", connections.to_string());
    for rule in &matching_data.rules {
        //println!("{}", rule.to_string());
        connections.apply(rule, &matching_data.elements);
        //println!("{}", connections.to_string());
    }

    debug_println!("preferences:");
    debug_println!("{}", connections.to_string());

    let mut possible_connections = connections.possible_connections();
    possible_connections.sort_by(|con1, con2| con1.len().cmp(&con2.len()));
    let (max_score, max_combinations) = calc_max_combinations(
        &connections,
        &possible_connections,
        HashSet::new(),
        &matching_data.outputs,
    );
    debug_println!("max score: {}", max_score);
    for comb in max_combinations {
        debug_println!("{:?}", comb);
    }
    possible_connections.sort_by(|con1, con2| con2.len().cmp(&con1.len()));
    let (max_score, max_combinations) = calc_max_combinations(
        &connections,
        &possible_connections,
        HashSet::new(),
        &matching_data.outputs,
    );
    debug_println!("max score: {}", max_score);
    for comb in &max_combinations {
        debug_println!("{:?}", comb);
    }
    return MatchingResult {
        score: max_score,
        connections: max_combinations,
    };
}
