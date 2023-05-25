use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div>
            <h1 class="text-lg">{ "404 NOT FOUND" }</h1>
        </div>
    }
}
