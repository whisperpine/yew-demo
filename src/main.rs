use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state_eq(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <div class={classes!("container")}>
            <button class="btn btn--secondary hover:shadow" onclick={onclick}>{ "+1" }</button>
            <p>{ *counter }</p>
            <p><abbr title="content delivery network">{"CDN"}</abbr></p>
            <button class="btn btn--primary hover:shadow">{ "Error" }</button>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
