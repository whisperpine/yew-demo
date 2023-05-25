use yew::prelude::*;

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
    use std::rc::Rc;
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
