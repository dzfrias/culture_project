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
                <>
                    <div>
                        <p>{ year }</p>
                    </div>
                    <div class={classes!("line")}></div>
                </>
            } else {
                <>
                    <details class={classes!("event-card")}>
                        <summary>{ year }</summary>
                        <p>{ event }</p>
                    </details>
                    <div class={classes!("line")}></div>
                </>
            }
        }
    });
    html! {
        <div class={classes!("card-group")}>
            { for cards }
        </div>
    }
}
