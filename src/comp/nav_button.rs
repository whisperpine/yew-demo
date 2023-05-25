use crate::Theme;
use std::rc::Rc;
use yew::prelude::*;

#[function_component]
pub fn NavButton() -> Html {
    let theme = use_context::<Rc<Theme>>().unwrap();

    html! {
        <button class="text-5xl rounded shadow p-10">{theme.foreground.clone()}</button>
    }
}
