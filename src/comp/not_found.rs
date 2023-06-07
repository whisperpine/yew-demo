use yew::prelude::*;

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div class="container mx-auto">
            <h1 class="text-3xl">{ "404 NOT FOUND" }</h1>
        </div>
    }
}
