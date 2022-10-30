use crate::breathing::Breathing;
use crate::patterns::{Pattern, PatternStep};

use lazy_static::lazy_static;
use std::time::Duration;

lazy_static! {
    pub static ref PATTERN: Pattern = Pattern {
        name: "relaxing breath".to_owned(),
        steps: vec![
            PatternStep {
                breathing: Breathing::Inhale,
                duration: Duration::from_secs(4),
            },
            PatternStep {
                breathing: Breathing::Hold,
                duration: Duration::from_secs(7),
            },
            PatternStep {
                breathing: Breathing::Exhale,
                duration: Duration::from_secs(8),
            },
        ]
    };
}
