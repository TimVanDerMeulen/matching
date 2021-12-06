use crate::rules::RuleSeverity;
use crate::rules::RuleSeverity::*;

pub(crate) trait Scorer<T> {
    fn get_score(&self) -> T;
}

impl Scorer<i8> for RuleSeverity {
    fn get_score(&self) -> i8 {
        return match self {
            Force => i8::MAX,
            Prefer => 2,
            Standard => 1,
            PreferExclude => -2,
            ForceExclude => i8::MIN,
        };
    }
}
