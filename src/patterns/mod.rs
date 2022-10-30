use crate::breathing::Breathing;
pub mod examples;

use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PatternStep {
    pub breathing: Breathing,
    pub duration: Duration,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub steps: Vec<PatternStep>,
}
