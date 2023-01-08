use js_sys::Array;
use std::{collections::HashMap, ops::Range};
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub range: Range<u32>,
    pub year_data: HashMap<u32, String>,
    #[prop_or_default]
    pub top_margin: i32,
    pub id: AttrValue,
}

#[function_component(Cards)]
pub fn cards(props: &Props) -> Html {
    let cards = props.range.clone().map(|year| {
        // Declared so it doesn't go out of scope as soon as its declared later
        let empty = String::new();
        let event = props.year_data.get(&year).unwrap_or(&empty);
        html! {
            if event.is_empty() {
                <div class={classes!("border-white", "border", "p-2")}>
                    <p class={classes!("text-white")}>{ year }</p>
                </div>
            } else {
                <div class={classes!("border-white", "border", "h-[30vh]", "w-[80vw]", "md:w-[50vw]", "flex", "justify-center", "items-center", "flex-col", "p-5", "_has_event")}>
                    <p class={classes!("text-white", "flex-auto", "text-xl")}>{ year }</p>
                    <p class={classes!("text-white", "flex-[3_3_0%]", "text-center")}>{ event }</p>
                </div>
            }
        }
    });

    {
        let id = props.id.clone();
        let top_margin = props.top_margin;
        // Run after render
        use_effect(move || {
            let callback = Closure::<dyn Fn(Array)>::new(|entries: Array| {
                // Find the last element farthest down element that can be highlighted
                let val = entries.reverse().find(&mut |val, _, _| {
                    let entry = val
                        .dyn_into::<IntersectionObserverEntry>()
                        .expect("should be an IntersectionObserverEntry");
                    let has_event = entry.target().class_list().contains("_has_event");
                    if !entry.is_intersecting() && has_event {
                        entry
                            .target()
                            .class_list()
                            .replace("border-yellow-200", "border-white")
                            .unwrap();
                    }
                    entry.is_intersecting() && has_event
                });
                // Will be undefined if is_intersecting() returns false
                if val.is_undefined() {
                    return;
                }
                let target_classes = val
                    .dyn_into::<IntersectionObserverEntry>()
                    .expect("should be an IntersectionObserverEntry")
                    .target()
                    .class_list();
                // Highlight yellow
                target_classes
                    .replace("border-white", "border-yellow-200")
                    .unwrap();
            });
            // Set up observer to observe each child
            let mut opts = IntersectionObserverInit::new();
            opts.threshold(&1.into());
            opts.root_margin(&format!("{}px 0px 0px 0px", top_margin));
            let observer =
                IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &opts)
                    .unwrap_throw();
            let children = gloo::utils::document()
                .get_element_by_id(&id)
                .expect("should have element with id")
                .children();
            for idx in 0..children.length() {
                let elem = children.item(idx).expect("should have element");
                observer.observe(&elem);
            }
            callback.forget();
        });
    }

    html! {
        <div class={classes!("flex", "flex-col", "items-center", "gap-10", "my-8")} id={&props.id}>
            { for cards }
        </div>
    }
}
