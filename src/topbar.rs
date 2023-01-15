use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub title: AttrValue,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(TopBar)]
pub fn top_bar(props: &Props) -> Html {
    let bar = use_node_ref();
    let mut last_scroll = gloo::utils::window().scroll_y().unwrap();
    let on_scroll = {
        let bar_ref = bar.clone();
        Closure::wrap(Box::new(move |_: Event| {
            let bar = bar_ref
                .cast::<HtmlElement>()
                .expect("should be an html element");
            let scroll = gloo::utils::window().scroll_y().unwrap();
            if last_scroll < scroll {
                bar.style()
                    .set_property("top", &((-bar.offset_height()).to_string() + "px"))
                    .unwrap();
            } else {
                bar.style().set_property("top", "0").unwrap();
            }
            last_scroll = scroll;
        }) as Box<dyn FnMut(_)>)
    };
    gloo::utils::window()
        .add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref())
        .unwrap_throw();
    on_scroll.forget();

    html! {
        <div id="top-bar" ref={bar}>
            <h1>{ &props.title }</h1>
            { for props.children.iter() }
        </div>
    }
}
