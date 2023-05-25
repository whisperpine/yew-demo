use yew::prelude::*;
use yew_demo::route::{switch, Route};
use yew_router::prelude::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </ BrowserRouter>
    }
}
