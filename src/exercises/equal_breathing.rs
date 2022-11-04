use crate::breathe::Breathe;
use crate::exercises::step::Step;
use crate::exercises::Exercise;

use lazy_static::lazy_static;
use std::time::Duration;

lazy_static! {
    pub static ref EXERCISE: Exercise = Exercise {
        name: "Equal Breathing".to_owned(),
        description: "".to_owned(),
        steps: vec![
            Step {
                breathing: Breathe::Inhale,
                duration: Duration::from_secs(4),
            },
            Step {
                breathing: Breathe::Exhale,
                duration: Duration::from_secs(4),
            },
        ]
    };
}
