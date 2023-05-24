use std::rc::Rc;
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

#[derive(PartialEq, Properties, Clone)]
pub struct ListItemProps {
    #[prop_or(AttrValue::from("nice"))]
    pub value: AttrValue,
}

#[function_component]
pub fn ListItem(props: &ListItemProps) -> Html {
    html! {
        <li>{props.value.clone()}</li>
    }
}

#[derive(PartialEq, Properties)]
pub struct DemoTableProps {
    pub children: ChildrenWithProps<ListItem>,
}

#[function_component]
pub fn DemoTable(props: &DemoTableProps) -> Html {
    // modify children's property
    let modified_children = props.children.iter().map(|mut item| {
        let mut list_props = Rc::make_mut(&mut item.props);
        list_props.value = AttrValue::from(format!("item-{}", list_props.value));
        item
    });

    html! {
        <table>
            {for modified_children}
        </table>
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Theme {
    pub foreground: String,
    pub background: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            foreground: String::from("foreground"),
            background: String::from("background"),
        }
    }
}

#[function_component]
pub fn NavButton() -> Html {
    let theme = use_context::<Rc<Theme>>().unwrap();

    html! {
        <button class="text-5xl rounded shadow p-10">{theme.foreground.clone()}</button>
    }
}

#[function_component]
pub fn NotFound() -> Html {
    html! {
        <div>
            <h1>{ "404 NOT FOUND" }</h1>
        </div>
    }
}

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
