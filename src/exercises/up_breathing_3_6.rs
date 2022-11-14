use crate::breathe::Breathe;
use crate::exercises::step::Step;
use crate::exercises::Exercise;

use lazy_static::lazy_static;
use std::time::Duration;

lazy_static! {
    pub static ref EXERCISE: Exercise = Exercise {
        name: "Up Breathing (3-6)".to_owned(),
        description: "".to_owned(),
        steps: vec![
            Step {
                breathing: Breathe::Inhale,
                duration: Duration::from_secs(3),
            },
            Step {
                breathing: Breathe::Exhale,
                duration: Duration::from_secs(6),
            },
        ]
    };
}
