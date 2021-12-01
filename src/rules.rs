use serde::{Deserialize, Serialize};
use derivative::Derivative;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub(crate) struct Rules {
    references: HashMap<String, String>,
    matching: Vec<Rule>,
}

#[derive(Derivative)]
#[derive(Serialize, Deserialize)]
struct Rule {
    #[derivative(Default(value = "RuleType::IGNORE"))]
    _type: RuleType,
    #[derivative(Default(value = "None"))]
    prio: Option<u64>,
    #[derivative(Default(value = "RuleOperand::MATCH"))]
    operand: RuleOperand,
    #[derivative(Default(value = ""))]
    field: String,
}

#[derive(Serialize, Deserialize)]
enum RuleType {
    Force, Prefer, ForceExclude, PreferExclude, Ignore
}

#[derive(Serialize, Deserialize)]
enum RuleOperand {
    Match, Include
}