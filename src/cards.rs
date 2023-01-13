use js_sys::Array;
use std::{collections::HashMap, ops::Range};
use wasm_bindgen::{prelude::Closure, JsCast, UnwrapThrowExt};
use web_sys::{Element, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub range: Range<u32>,
    pub year_data: HashMap<u32, String>,
    pub id: AttrValue,
    pub next_card: Callback<Element>,
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
                <div class={classes!("border-white", "border", "h-[200px]", "w-[min(400px,80vw)]", "flex", "justify-center", "items-center", "flex-col", "p-5", "_has_event")}>
                    <p class={classes!("text-white", "flex-auto", "text-xl")}>{ year }</p>
                    <p class={classes!("text-white", "flex-[3_3_0%]", "text-center")}>{ event }</p>
                </div>
            }
        }
    });

    // Stack of elements in viewport
    let in_viewport: UseListHandle<Element> = use_list(Vec::new());
    let v = in_viewport.current().to_owned();
    if !v.is_empty() {
        props.next_card.emit(v.last().unwrap().clone());
    }

    {
        let id = props.id.clone();
        use_effect_once(move || {
            let callback = Closure::<dyn Fn(Array)>::new(move |entries: Array| {
                entries.for_each(&mut |entry, _, _| {
                    let entry = entry.dyn_into::<IntersectionObserverEntry>().unwrap();
                    if entry.is_intersecting() {
                        in_viewport.push(entry.target())
                    } else {
                        in_viewport.pop();
                    }
                })
            });
            // Set up observer to observe each child
            let mut opts = IntersectionObserverInit::new();
            opts.threshold(&1.into());
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
            || ()
        });
    }

    html! {
        <div class={classes!("flex", "flex-col", "items-center", "gap-10", "my-8")} id={&props.id}>
            { for cards }
        </div>
    }
}
