use crate::connections::Connection;
use crate::score::Scorer;
use derivative::Derivative;
use regex;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Derivative, Serialize, Deserialize, Clone)]
pub(crate) struct Rule {
    #[derivative(Default(value = "RuleSeverity::IGNORE"))]
    pub(crate) severity: RuleSeverity,
    #[derivative(Default(value = ""))]
    pub(crate) field: String,
    #[derivative(Default(value = ""))]
    pub(crate) target_field: String,
    #[derivative(Default(value = "RuleOperand::MATCH"))]
    pub(crate) operand: RuleOperand,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub(crate) enum RuleSeverity {
    Force,
    Prefer,
    Standard,
    PreferExclude,
    ForceExclude,
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq)]
pub(crate) enum RuleOperand {
    Match,
    Include,
}

pub(crate) trait RuleActions {
    fn check(
        &self,
        id: &str,
        target: &str,
        values: &HashMap<String, HashMap<String, String>>,
    ) -> bool;
    fn apply(&self, id: &str, target: &str, connections: &mut HashMap<String, Vec<Connection>>);
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

    fn apply(&self, id: &str, target: &str, connections: &mut HashMap<String, Vec<Connection>>) {
        match self.severity {
            RuleSeverity::Force
            | RuleSeverity::Prefer
            | RuleSeverity::Standard
            | RuleSeverity::PreferExclude => {
                let mut con = Connection {
                    to_element: target.to_string(),
                    score: self.severity.getScore(),
                };
                connections
                    .get_mut(id)
                    .expect(&*format!("Missing element {}", id))
                    .push(con);
            }
            RuleSeverity::ForceExclude => {
                // remove any connection between both
                connections
                    .get_mut(id)
                    .expect(&*format!("Missing element {}", id))
                    .retain(|c| c.to_element == target);
                connections
                    .get_mut(target)
                    .expect(&*format!("Missing element {}", target))
                    .retain(|c| c.to_element == id);
            }
        }
    }
}
