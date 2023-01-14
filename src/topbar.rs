use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub title: AttrValue,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(TopBar)]
pub fn top_bar(props: &Props) -> Html {
    html! {
        <div id="top-bar">
            <h1>{ &props.title }</h1>
            { for props.children.iter() }
        </div>
    }
}
