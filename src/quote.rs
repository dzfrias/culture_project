use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, IntersectionObserver};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub top: u32,
    pub left: u32,
    #[prop_or_default]
    pub observer: Option<IntersectionObserver>,
}

fn get_overlay() -> HtmlElement {
    gloo::utils::document()
        .get_element_by_id("overlay")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap()
}

#[function_component(Quote)]
pub fn quote(props: &Props) -> Html {
    let quote_ref = use_node_ref();

    {
        let quote_ref = quote_ref.clone();
        let observer = props.observer.clone();
        use_effect_once(move || {
            if let Some(observer) = observer {
                let quote = quote_ref.cast::<Element>().unwrap();
                observer.observe(&quote);
            }
            || ()
        })
    }

    let window = use_window_size();
    let expanded = use_bool_toggle(true);

    let onclick = {
        let expanded = expanded.clone();
        let quote_ref = quote_ref.clone();
        Callback::from(move |event: MouseEvent| {
            if window.0 > 600.0 || event.offset_x() > 10 {
                return;
            }
            let classes = quote_ref.cast::<HtmlElement>().unwrap().class_list();
            if *expanded {
                classes.remove_1("block").unwrap();
                get_overlay()
                    .style()
                    .set_property("visibility", "visible")
                    .unwrap();
            } else {
                classes.add_1("block").unwrap();
                get_overlay()
                    .style()
                    .set_property("visibility", "hidden")
                    .unwrap();
            }
            expanded.toggle();
        })
    };

    use_effect(move || {
        if window.0 > 600.0 {
            get_overlay()
                .style()
                .set_property("visibility", "hidden")
                .unwrap();
        }
    });

    html! {
        <blockquote {onclick} ref={quote_ref} style={format!("top: {}rem; left: {}vw;", props.top, props.left)} class={classes!("quote", if window.0 < 600.0 { "block" } else { "" })}>
            { for props.children.iter() }
        </blockquote>
    }
}
