use crate::exercises::Exercise;
use crate::Route;

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(IndexComponent)]
pub fn html() -> Html {
    html! {
        <main class="container">
            <h2>{ "Exercises" }</h2>
            <p> { r"
                Breathing exercises is when we intentionally control how to
                breathe to experience a sense of relaxation and calm in the
                mind and body.
            " } </p>
            <p> { r"
                Bellow are a few that you can use to respond to stress in a
                more skillful way.
            " } </p>
            <ul>
                {
                    Exercise
                        ::all()
                        .into_iter()
                        .map(|exercise| html! {
                            <li><Link<Route> to={Route::Animation { slug: exercise.slug() }}>{ exercise.name }</Link<Route>></li>
                        })
                        .collect::<Html>()
                }
            </ul>
        </main>
    }
}
