mod box_breathing;
mod equal_breathing;
mod pursed_lip_breathing;
mod relaxing_breath;
mod resonance_breathing;

use crate::patterns::Pattern;

pub fn all() -> Vec<Pattern> {
    vec![
        box_breathing::PATTERN.clone(),
        equal_breathing::PATTERN.clone(),
        relaxing_breath::PATTERN.clone(),
        pursed_lip_breathing::PATTERN.clone(),
        resonance_breathing::PATTERN.clone(),
    ]
}
