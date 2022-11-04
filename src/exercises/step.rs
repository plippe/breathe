use crate::breathe::Breathe;

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Step {
    pub breathing: Breathe,
    pub duration: Duration,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StepContext {
    pub breathing: Breathe,
    pub start: Duration,
    pub start_percentage: f64,
    pub end: Duration,
    pub end_percentage: f64,
}

impl StepContext {
    const EMPTY: StepContext = StepContext {
        breathing: Breathe::Inhale,
        start: Duration::from_secs(0),
        start_percentage: 0.0,
        end: Duration::from_secs(0),
        end_percentage: 0.0,
    };

    pub fn from_steps(steps: &[Step]) -> Vec<StepContext> {
        let duration = steps
            .iter()
            .map(|it| it.duration)
            .sum::<Duration>()
            .as_secs_f64();

        steps.iter().fold(vec![], {
            |mut acc: Vec<StepContext>, step| {
                let previous = acc.last().unwrap_or(&StepContext::EMPTY);

                let start = previous.end;
                let end = previous.end + step.duration;

                let current = StepContext {
                    breathing: step.breathing,
                    start,
                    start_percentage: 100.0 * start.as_secs_f64() / duration,
                    end,
                    end_percentage: 100.0 * end.as_secs_f64() / duration - 0.001,
                };

                acc.push(current);
                acc
            }
        })
    }
}
