use crate::rules::{Rule, RuleActions, RuleSeverity};
use crate::score::Scorer;
use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;

pub(crate) struct Connections {
    matrix: Vec<Vec<i16>>,
    fixed_order: Vec<String>,
}

impl Connections {
    pub(crate) fn len(&self) -> usize {
        return self.fixed_order.len();
    }
}

pub(crate) trait Connector<S, T> {
    fn from_data(data: &T) -> Self;
    fn apply(&mut self, to_apply: &S, extra_info: &T);
    fn calc_score(&self, group: &Vec<&usize>) -> (bool, i32);
    fn possible_connections(&self) -> Vec<Vec<usize>>;
    //fn find_highest_score(&self, possible_connections: &Vec<Vec<i16>>) -> (i64, Vec<Vec<i16>>);
}

impl Connector<Rule, HashMap<String, HashMap<String, String>>> for Connections {
    fn from_data(data: &HashMap<String, HashMap<String, String>>) -> Connections {
        let mut fixed_order = Vec::from_iter(data.keys().cloned());
        fixed_order.sort();
        let size = data.len();
        let mut matrix = vec![vec![RuleSeverity::Standard.get_score() as i16; size]; size];
        for index in 0..size {
            matrix[index][index] = i16::MIN;
        }

        return Connections {
            matrix,
            fixed_order,
        };
    }

    fn apply(&mut self, rule: &Rule, fields: &HashMap<String, HashMap<String, String>>) {
        let size = self.fixed_order.len();
        for x in 0..size {
            for y in 0..size {
                match rule.severity {
                    RuleSeverity::Force | RuleSeverity::ForceExclude => {}
                    _ => {
                        if self.matrix[x][y] == i16::MIN || self.matrix[y][x] == i16::MIN {
                            continue;
                        }
                    }
                }
                rule.check_and_apply(
                    &*self.fixed_order[x],
                    &*self.fixed_order[y],
                    fields,
                    x,
                    y,
                    &mut self.matrix,
                );
            }
        }
    }

    fn calc_score(&self, group: &Vec<&usize>) -> (bool, i32) {
        let mut score: i32 = 0;
        let size = group.len();
        for x in 0..size {
            for y in 0..size {
                if x == y {
                    continue;
                }
                if self.matrix[*group[x]][*group[y]] == i16::MIN {
                    return (false, i32::MIN);
                }
                score += self.matrix[*group[x]][*group[y]] as i32;
            }
        }
        return (true, score);
    }

    fn possible_connections(&self) -> Vec<Vec<usize>> {
        let size = self.fixed_order.len();
        let mut possible_connections: Vec<Vec<usize>> = Vec::new();
        for row in 0..size {
            let mut connections: Vec<usize> = Vec::new();
            connections.push(row);
            for col in 0..size {
                if row != col
                    && self.matrix[row][col] != i16::MIN
                    && self.matrix[col][row] != i16::MIN
                {
                    connections.push(col);
                }
            }
            possible_connections.push(connections);
        }
        return possible_connections;
    }
}

impl fmt::Display for Connections {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let size = 6;
        write!(f, "{0: <width$}", " ", width = size).expect("issue while printing connection");
        for element in &self.fixed_order {
            write!(f, " | {0: <width$}", element, width = size)
                .expect("issue while printing connection");
        }
        writeln!(f).expect("issue while printing connection");
        writeln!(f, "{}", "-".repeat(10 * size)).expect("issue while printing connection");
        let size = self.matrix.len();
        for row in 0..size {
            write!(f, "{0: <width$}", self.fixed_order[row], width = size)
                .expect("issue while printing connection");
            for col in 0..size {
                write!(f, " | {0: >width$}", self.matrix[row][col], width = size)
                    .expect("issue while printing connection");
            }
            writeln!(f).expect("issue while printing connection");
        }
        writeln!(f)
    }
}
