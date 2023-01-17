use std::rc::Rc;
use web_sys::{Element, IntersectionObserver};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    pub top: u32,
    #[prop_or_default]
    pub observer: Option<Rc<IntersectionObserver>>,
    #[prop_or("end".into())]
    pub alignment: AttrValue,
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
        });
    }

    html! {
        <blockquote ref={quote_ref} style={format!("--top: {}%; --alignment: {};", props.top, props.alignment)} class={classes!("quote")}>
            { for props.children.iter() }
        </blockquote>
    }
}
