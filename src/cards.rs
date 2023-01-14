use std::{collections::HashMap, ops::Range};

use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub range: Range<u32>,
    pub year_data: HashMap<u32, String>,
}

#[function_component(Cards)]
pub fn cards(props: &Props) -> Html {
    let cards = props.range.to_owned().map(|year| {
        // Declared so it doesn't go out of scope as soon as its declared later
        let empty = String::new();
        let event = props.year_data.get(&year).unwrap_or(&empty);
        html! {
            if event.is_empty() {
                <div class={classes!("border-white", "border", "p-2")}>
                    <p class={classes!("text-white")}>{ year }</p>
                </div>
            } else {
                <div class={classes!("border-white", "border", "h-[200px]", "w-[min(400px,80vw)]", "flex", "justify-center", "items-center", "flex-col", "p-5")}>
                    <p class={classes!("text-white", "flex-auto", "text-xl")}>{ year }</p>
                    <p class={classes!("text-white", "flex-[3_3_0%]", "text-center")}>{ event }</p>
                </div>
            }
        }
    });
    html! {
        <div class={classes!("flex", "flex-col", "items-center", "gap-10", "my-8")}>
            { for cards }
        </div>
    }
}
