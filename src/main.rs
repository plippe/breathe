mod breathe;
mod exercises;
mod html;

use crate::exercises::Exercise;
use crate::html::animation::AnimationComponent;
use crate::html::index::IndexComponent;

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Index,
    #[at("/breathe/:slug")]
    Animation { slug: String },
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Index => html! { <IndexComponent /> },
        Route::Animation { slug } => Exercise::find_by_slug(slug).map_or_else(
            || html! { <Redirect<Route> to={Route::Index}/> },
            |exercise| html! { <AnimationComponent exercise={ exercise } /> },
        ),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
          <style>
            { r"
                body { font-family: Roboto; }
                a.contrast:focus { background-color: transparent; }
            " }
          </style>
          <nav class="container" style="position:relative; z-index: 1">
            <hgroup>
                <h1><Link<Route> to={Route::Index} classes="contrast">{ "Breathe" }</Link<Route>></h1>
                <h2></h2>
            </hgroup>
          </nav>
          <Switch <Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
