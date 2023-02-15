use yew::prelude::*;
use yew::props;
use yew_demo::*;

fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
fn App() -> Html {
    let nice_props = props! {
        yew_demo::NiceProps {
            is_loading: true,
            name: String::from("nice"),
        }
    };

    let cb = Callback::from(move |name: AttrValue| {
        let nice = AttrValue::from(name);
        web_sys::console::log_1(&nice.to_string().into());
    });

    let on_click = Callback::from(move |_| {
        web_sys::console::log_1(&"hello".into());
    });

    html! {
        <div class="">
            <Nice is_loading={true} />
            <Nice name={"yusong"} />
            <Nice ..nice_props />
            <Yahaha />
            <GreetBtn name="yahaha" cb={cb} />
            <button class="text-blue-400" onclick={on_click}>{ "hello" }</button>
            <button class="bg-gray-200"></button>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct AmiaoProps {}

#[function_component]
pub fn Amiao(props: &AmiaoProps) -> Html {
    let AmiaoProps {} = props;
    html! {
        <div></div>
    }
}
