use crate::matching::score::Scorer;
use derivative::Derivative;
use regex;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt;

#[derive(Derivative, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Rule {
    #[derivative(Default(value = "RuleSeverity::IGNORE"))]
    pub(crate) severity: RuleSeverity,
    #[derivative(Default(value = ""))]
    pub(crate) field: String,
    #[derivative(Default(value = ""))]
    pub(crate) target_field: String,
    #[derivative(Default(value = "RuleOperand::MATCH"))]
    pub(crate) operand: RuleOperand,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum RuleSeverity {
    Force,
    Prefer,
    Standard,
    PreferExclude,
    ForceExclude,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum RuleOperand {
    Match,
    Include,
}

pub trait RuleActions {
    fn check(
        &self,
        id: &str,
        target: &str,
        values: &HashMap<String, HashMap<String, String>>,
    ) -> bool;
    fn apply(&self, x: usize, y: usize, connections: &mut Vec<Vec<i16>>);
    fn check_and_apply(
        &self,
        id: &str,
        target: &str,
        values: &HashMap<String, HashMap<String, String>>,
        x: usize,
        y: usize,
        connections: &mut Vec<Vec<i16>>,
    );
}

impl RuleActions for Rule {
    fn check(
        &self,
        id: &str,
        target: &str,
        values: &HashMap<String, HashMap<String, String>>,
    ) -> bool {
        let value = values
            .get(id)
            .expect(&*format!("No values found for target {}", id))
            .get(&*self.field)
            .expect(&*format!(
                "Target {} has no value for {}",
                id, self.target_field
            ))
            .trim();
        let target_value = values
            .get(target)
            .expect(&*format!("No values found for target {}", target))
            .get(&*self.target_field)
            .expect(&*format!(
                "Target {} has no value for {}",
                target, self.target_field
            ))
            .trim();
        return match self.operand {
            RuleOperand::Match => value == target_value,
            RuleOperand::Include => Regex::new(&*format!(
                "^(.*, *)?{}( *,.*|$)",
                regex::escape(target_value)
            ))
            .unwrap()
            .is_match(value),
        };
    }

    fn apply(&self, x: usize, y: usize, connections: &mut Vec<Vec<i16>>) {
        match self.severity {
            RuleSeverity::Force | RuleSeverity::ForceExclude => connections[x][y] = i16::MIN, // same because force is inverted to exclude all others
            _ => connections[x][y] += self.severity.get_score() as i16,
        };
    }

    fn check_and_apply(
        &self,
        id: &str,
        target: &str,
        values: &HashMap<String, HashMap<String, String, RandomState>, RandomState>,
        x: usize,
        y: usize,
        connections: &mut Vec<Vec<i16>>,
    ) {
        let check = self.check(id, target, values);
        if match self.severity {
            RuleSeverity::Force => !check, // invert to exclude all non matching
            _ => check,
        } {
            self.apply(x, y, connections)
        }
    }
}
impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} to {} {}",
            self.severity, self.field, self.operand, self.target_field
        )
    }
}
impl fmt::Display for RuleSeverity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl fmt::Display for RuleOperand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
