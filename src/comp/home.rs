use crate::comp::*;
use crate::Theme;
use std::rc::Rc;
use yew::prelude::*;
use yew::props;

#[function_component]
pub fn Home() -> Html {
    use crate::comp::demo_span::DemoSpanProps;

    let demo_span_props = props!(DemoSpanProps {
        is_loading: true,
        name: AttrValue::from("nice"),
    });

    let on_click = Callback::from(move |_| {
        web_sys::console::log_1(&"hello".into());
    });

    let theme = use_memo(
        |_| crate::Theme {
            foreground: "yellow".to_owned(),
            background: "pink".to_owned(),
        },
        (),
    );

    html! {
        <div class="">
            <DemoSpan is_loading={true} />
            <DemoSpan name={"yusong"} />
            <DemoSpan ..demo_span_props />

            <button class="text-white bg-blue-400 shadow-lg rounded-lg p-5 hover:shadow-sm" onclick={on_click}>
                { "hello" }
            </button>

            <DemoTable>
                <ListItem value={"rua"} />
                <ListItem />
                <ListItem />
            </DemoTable>

            <@{format!("h{}", 1)} class="text-3xl">{ "title" }</@>

            <ContextProvider<Rc<Theme>> context={theme}>
                <NavButton />
            </ContextProvider<Rc<Theme>>>
        </div>
    }
}
