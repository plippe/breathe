mod key_frames;

use crate::exercises::step::StepContext;
use crate::exercises::Exercise;
use crate::html::animation::key_frames::{KeyFrame, KeyFrames};

use yew::*;

#[derive(PartialEq, Properties)]
pub struct AnimationProps {
    pub exercise: Exercise,
}

#[function_component(AnimationComponent)]
pub fn html(props: &AnimationProps) -> Html {
    let steps = StepContext::from_steps(props.exercise.steps.as_slice());
    let duration = steps.last().map_or(0, |step| step.end.as_secs());

    let key_frames: Vec<KeyFrame> = steps
        .iter()
        .flat_map(KeyFrame::from_step_in_context)
        .collect();

    html! {
        <main class="container-flux">
            <div id="animation">
                <div id="animation-label"></div>
            </div>
            <style>
                { r"
                    #animation {
                        position:absolute;
                        top: 0;
                        left: 0;
                        height: 100%;
                        width: 100%;
                        display: flex;
                        justify-content: center;
                        align-items: center;
                    }

                    #animation-label:after {
                        color:#eee;
                        font-family: Sansita Swashed, cursive;
                        font-size: 7em;
                        content: '';
                        text-shadow: 0 0 0.01em #333;
                    }
                " }

                { format!("#animation {{ animation: animation {}s linear 0s infinite normal; }}", duration) }
                { KeyFrames { name: "animation".to_owned(), key_frames: key_frames.clone() }.as_css(&["background-color"]) }

                { format!("#animation-label {{ animation: animation-label {}s linear 0s infinite normal; }}", duration) }
                { KeyFrames { name: "animation-label".to_owned(), key_frames: key_frames.clone() }.as_css(&["transform"]) }

                { format!("#animation-label:after {{ animation: animation-label-after-animation {}s linear 0s infinite normal; }}", duration) }
                { KeyFrames { name: "animation-label-after-animation".to_owned(), key_frames: key_frames.clone() }.as_css(&["content"]) }
            </style>
        </main>
    }
}
