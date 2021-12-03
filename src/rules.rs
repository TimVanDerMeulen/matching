use connections::Connection;
use derivative::Derivative;
use regex;
use score::Scorer;
use serde::{Deserialize, Serialize};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

#[derive(Derivative, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub(crate) enum RuleSeverity {
    Force,
    Prefer,
    Standard,
    PreferExclude,
    ForceExclude,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum RuleOperand {
    Match,
    Include,
}

trait RuleActions {
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
            RuleOperand::Include => {
                value.matches(format!("^(.*, *)?{}( *,.*|$)", regex::escape(target_value)))
            }
        };
    }

    fn apply(&self, id: &str, target: &str, connections: &mut HashMap<String, Vec<Connection>>) {
        match self.severity {
            RuleSeverity::Force
            | RuleSeverity::Prefer
            | RuleSeverity::Standard
            | RuleSeverity::PreferExclude => {
                let mut con = Connection {
                    to_element: target.into_string(),
                    score: self.severity.getScore(),
                };
                connections
                    .get(id)
                    .expect(&*format!("Missing element {}", id))
                    .push(con);

                // increase score for bidirectional references
                // todo better logic and don't forget con from above
                //if self.field != self.target_field {
                //    let mut back_con = connections
                //        .get(target)
                //        .expect(&*format!("Missing element {}", target))
                //        .iter_mut()
                //        .find(|c| c.to_element == id);
                //    if let Some(bc) = back_con {
                //        if bc.score > 0 {
                //            bc.score += RuleSeverity::Standard.getScore()
                //        } else {
                //            bc.score -= RuleSeverity::Standard.getScore()
                //        }
                //    }
                //}
            }
            RuleSeverity::ForceExclude => {
                // remove any connection between both
                connections
                    .get(id)
                    .expect(&*format!("Missing element {}", id))
                    .retain(|&c| c.to_element == target);
                connections
                    .get(target)
                    .expect(&*format!("Missing element {}", target))
                    .retain(|&c| c.to_element == id);
            }
        }
    }
}

// trait RuleApplier<T> {
//     fn apply(&self, id: String, apply_to: &mut T, target: String, apply_to: &mut T);
// }
//
// impl RuleApplier<Vec<Connection>> for Rule {
//     fn apply(&self, id: String, cons: &mut Vec<Connection>) {
//         let mut locked = matches!(self.severity, RuleSeverity::Force);
//         for con in cons {
//             match self.operand {
//                 RuleOperand::Match => {}
//                 RuleOperand::Include => {}
//             }
//         }
//
//         cons.push(Connection { to_element: id, score: self.severity.getScore(), locked: matches!(self.severity, RuleSeverity::Force) });
//     }
// }
