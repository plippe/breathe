mod box_breathing;
mod equal_breathing;
pub mod step;

use crate::exercises::step::Step;

#[derive(Debug, Clone, PartialEq)]
pub struct Exercise {
    pub name: String,
    pub description: String,
    pub steps: Vec<Step>,
}

impl Exercise {
    pub fn all() -> Vec<Exercise> {
        vec![
            box_breathing::EXERCISE.clone(),
            equal_breathing::EXERCISE.clone(),
        ]
    }

    pub fn find_by_slug(slug: &str) -> Option<Exercise> {
        Self::all().iter().find(|e| e.slug() == slug).cloned()
    }

    pub fn slug(&self) -> String {
        self.name.to_lowercase().replace(' ', "-")
    }
}
