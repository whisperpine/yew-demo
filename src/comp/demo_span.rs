use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct DemoSpanProps {
    #[prop_or_default]
    pub is_loading: bool,
    #[prop_or(AttrValue::from("amiao"))]
    pub name: AttrValue,
}

#[function_component]
pub fn DemoSpan(props: &DemoSpanProps) -> Html {
    let DemoSpanProps { is_loading, name } = props;
    html! {
        <span class="text-yellow-600">
            {"Am I loading? - "}{is_loading}{" - "}{name}
            <br/>
        </span>
    }
}
