use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub top: i32,
    pub text: AttrValue,
    #[prop_or("end".into())]
    pub alignment: AttrValue,
}

#[function_component(Quote)]
pub fn quote(props: &Props) -> Html {
    html! {
        <blockquote style={format!("--top: {}%; --alignment: {};", props.top, props.alignment)} class={classes!("quote")}>
            <p>{ &props.text }</p>
        </blockquote>
    }
}
