use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct NewsProps {
    pub id: u8,
}

#[function_component]
pub fn News(props: &NewsProps) -> Html {
    let NewsProps { id } = props;
    html! {
        <div>
            <h1 class="text-lg">{ "News ID: " } {id}</h1>
        </div>
    }
}
