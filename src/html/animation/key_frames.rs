use crate::breathe::Breathe;
use crate::exercises::step::StepContext;

use std::collections::HashMap;

pub struct KeyFrames {
    pub name: String,
    pub key_frames: Vec<KeyFrame>,
}

impl KeyFrames {
    pub fn as_css(&self, property_names: &[&str]) -> String {
        let key_frames = self
            .key_frames
            .iter()
            .map(|key_frame| key_frame.as_css(property_names))
            .collect::<Vec<String>>()
            .join("\n");

        format!("@keyframes {} {{\n {} \n}}", self.name, key_frames)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyFrame {
    pub percent: f64,
    pub properties: HashMap<String, String>,
}

impl KeyFrame {
    pub fn from_step_in_context(step: &StepContext) -> Vec<KeyFrame> {
        match step.breathing {
            Breathe::Inhale => vec![
                KeyFrame {
                    percent: step.start_percentage,
                    properties: HashMap::from([
                        ("content".to_owned(), "'inhale'".to_owned()),
                        ("background-color".to_owned(), "#ffb000".to_owned()),
                        ("transform".to_owned(), "scale(0.5)".to_owned()),
                    ]),
                },
                KeyFrame {
                    percent: step.end_percentage,
                    properties: HashMap::from([
                        ("content".to_owned(), "'inhale'".to_owned()),
                        ("background-color".to_owned(), "#dc267f".to_owned()),
                        ("transform".to_owned(), "scale(1)".to_owned()),
                    ]),
                },
            ],
            Breathe::Exhale => vec![
                KeyFrame {
                    percent: step.start_percentage,
                    properties: HashMap::from([
                        ("content".to_owned(), "'exhale'".to_owned()),
                        ("background-color".to_owned(), "#dc267f".to_owned()),
                        ("transform".to_owned(), "scale(1)".to_owned()),
                    ]),
                },
                KeyFrame {
                    percent: step.end_percentage,
                    properties: HashMap::from([
                        ("content".to_owned(), "'exhale'".to_owned()),
                        ("background-color".to_owned(), "#ffb000".to_owned()),
                        ("transform".to_owned(), "scale(0.5)".to_owned()),
                    ]),
                },
            ],
            Breathe::Hold => vec![
                KeyFrame {
                    percent: step.start_percentage,
                    properties: HashMap::from([("content".to_owned(), "'pause'".to_owned())]),
                },
                KeyFrame {
                    percent: step.end_percentage,
                    properties: HashMap::from([("content".to_owned(), "'pause'".to_owned())]),
                },
            ],
        }
    }

    pub fn as_css(&self, property_names: &[&str]) -> String {
        let percent = if self.percent == 0.0 {
            "0%, 100%".to_owned()
        } else {
            format!("{}%", self.percent)
        };

        let properties = self
            .properties
            .iter()
            .filter(|(key, _)| property_names.contains(&key.as_str()))
            .map(|(key, value)| format!("{}: {}", key, value))
            .collect::<Vec<String>>()
            .join(";");

        format!("{} {{ {} }}", percent, properties)
    }
}
