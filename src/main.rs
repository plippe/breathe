mod breathing;
mod patterns;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Hello World" }</h1>
            <ul>
                {
                    patterns
                        ::examples
                        ::all()
                        .into_iter()
                        .map(|example| html! {
                            <li>{ example.name }</li>
                        })
                        .collect::<Html>()
                }
            </ul>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
