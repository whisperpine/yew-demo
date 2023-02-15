use yew::prelude::*;

pub struct Yahaha;

impl Component for Yahaha {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <p class="text-red-500">{ "yahaha" }</p>
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct GreetBtnProps {
    pub name: AttrValue,
    pub cb: Callback<AttrValue>,
}

#[function_component]
pub fn GreetBtn(props: &GreetBtnProps) -> Html {
    let GreetBtnProps { name, cb } = props;
    cb.emit(name.clone());

    html! {
        <button></button>
    }
}

#[derive(PartialEq, Properties)]
pub struct NiceProps {
    #[prop_or_default]
    pub is_loading: bool,
    #[prop_or(AttrValue::from("amiao"))]
    pub name: AttrValue,
}

#[function_component]
pub fn Nice(props: &NiceProps) -> Html {
    let NiceProps { is_loading, name } = props;
    html! {
        <div class="text-yellow-600">{"Am I loading? - "}{is_loading}{" - "}{name}</div>
    }
}
