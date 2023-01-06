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
        <div class={classes!("sticky", "flex", "items-center", "gap-3", "border-b", "border-white", "text-white", "pb-4", "flex-col", "bg-neutral-800")}>
            <h1 class={classes!("text-2xl")}>{ &props.title }</h1>
            { for props.children.iter() }
        </div>
    }
}
