mod cards;
mod chart;
mod data_button;
mod json_load;
mod quote;

use cards::Cards;
use chart::{Chart, Dataset};
use data_button::DataButton;
use js_sys::Array;
use json_load::{Alignment, Chart as ChartJson, Quotes as QuotesJson, Side};
use quote::Quote;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    HtmlElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};
use yew::{prelude::*, props};

fn get_datasets(s: &str) -> (Vec<Dataset>, Vec<JsValue>) {
    let chart: ChartJson = serde_json::from_str(s).expect("Should have valid chart data");
    let labels = (chart.start..=chart.end)
        .step_by(chart.step)
        .map(|num| num.into())
        .collect::<Vec<_>>();
    let datasets = chart
        .datasets
        .into_iter()
        .map(|dataset| {
            props! {
                Dataset {
                    label: dataset.label,
                    data: dataset.data,
                    bg_color: dataset.bg_color,
                    border_color: dataset.border_color,
                }
            }
        })
        .collect::<Vec<_>>();
    (datasets, labels)
}

#[function_component(App)]
fn app() -> Html {
    let (pop_data, pop_labels) = get_datasets(include_str!("../static/charts/test.json"));
    let (fertility_data, fertility_labels) =
        get_datasets(include_str!("../static/charts/test2.json"));

    let datasets = use_state(|| pop_data.clone());
    let labels = use_state(|| pop_labels.clone());
    let callback = {
        let datasets = datasets.clone();
        let labels = labels.clone();
        Callback::from(move |(data, data_lables): (Vec<Dataset>, Vec<JsValue>)| {
            datasets.set(data);
            labels.set(data_lables);
        })
    };

    let quote_observer = {
        let callback = Closure::wrap(Box::new(move |entries: Array| {
            entries.for_each(&mut |entry, _, _| {
                let entry = entry.dyn_into::<IntersectionObserverEntry>().unwrap();
                if entry.is_intersecting() {
                    entry
                        .target()
                        .dyn_into::<HtmlElement>()
                        .unwrap()
                        .style()
                        .set_property("opacity", "1")
                        .unwrap();
                } else {
                    entry
                        .target()
                        .dyn_into::<HtmlElement>()
                        .unwrap()
                        .style()
                        .set_property("opacity", "0.6")
                        .unwrap();
                }
            })
        }) as Box<dyn FnMut(_)>);
        let mut opts = IntersectionObserverInit::new();
        opts.root_margin("-53% 0 -10% 0");
        let observer =
            IntersectionObserver::new_with_options(callback.as_ref().unchecked_ref(), &opts)
                .unwrap();
        callback.forget();
        Rc::new(observer)
    };

    let mut bar1_quotes = Vec::new();
    let mut bar2_quotes = Vec::new();
    serde_json::from_str::<QuotesJson>(include_str!("../static/quotes/quotes.json"))
        .expect("should have valid dataset")
        .quotes
        .into_iter()
        .for_each(|quote| {
            let alignment = match quote.alignment {
                Alignment::Start => "start",
                Alignment::Center => "center",
                Alignment::End => "end",
            };
            let node = html! {
                <Quote top={quote.top} {alignment} observer={quote_observer.clone()}>
                    <p>{ quote.text }</p>
                </Quote>
            };
            if quote.side == Side::Left {
                bar1_quotes.push(node);
            } else {
                bar2_quotes.push(node);
            }
        });

    html! {
        <>
            <div class="navbar">
                <h1>{ "The One Child Policy" }</h1>
                <span>{ "独生子女政策" }</span>
            </div>
            <h1><a href="#charts">{ "Chart" }</a></h1>
            <hr/>
            <div class={classes!("center")} >
                <div class={classes!("side-by-side", "min-center")}>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>)> text="Population" data={(pop_data, pop_labels)} callback={callback.clone()}/>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>)> text="Fertility" data={(fertility_data, fertility_labels)} {callback}/>
                </div>
                <Chart
                    id="main-chart"
                    datasets={(*datasets).clone()}
                    labels={(*labels).clone()}
                />
            </div>
            <h1><a href="#timeline">{ "Timeline" }</a></h1>
            <hr/>
            <div class={classes!("side-by-side")}>
                <div class={classes!("quote-bar")}>
                    { for bar1_quotes }
                </div>
                <Cards
                    range={1980..2017}
                    year_data={
                        serde_json::from_str::<HashMap<u32, String>>(
                            include_str!("../static/year_data/year_data.json"))
                                .expect("should have valid json")
                    }
                />
                <div class={classes!("quote-bar")}>
                    { for bar2_quotes }
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
