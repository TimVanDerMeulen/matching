use crate::rules::RuleSeverity;
use crate::rules::RuleSeverity::*;

pub(crate) trait Scorer<T> {
    fn getScore(&self) -> T;
}

impl Scorer<i8> for RuleSeverity {
    fn getScore(&self) -> i8 {
        return match self {
            Force=> 99,
            Prefer => 2,
            Standard => 1,
            PreferExclude => -2,
            ForceExclude => -99
        }
    }
}