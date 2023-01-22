mod cards;
mod chart;
mod data_button;
mod json_load;
mod quote;

use cards::Cards;
use chart::{Chart, Dataset};
use data_button::DataButton;
use js_sys::Array;
use json_load::Chart as ChartJson;
use quote::Quote;
use std::{collections::HashMap, rc::Rc};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{
    HtmlElement, IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit,
};
use yew::{prelude::*, props};
use yew_hooks::prelude::*;

fn get_datasets(s: &str) -> (Vec<Dataset>, Vec<JsValue>, JsValue) {
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
    let opts = serde_wasm_bindgen::to_value(&chart.opts).unwrap();
    (datasets, labels, opts)
}

#[function_component(App)]
fn app() -> Html {
    let (pop_data, pop_labels, pop_opts) = get_datasets(include_str!("../static/charts/test.json"));
    let (fertility_data, fertility_labels, fertility_opts) =
        get_datasets(include_str!("../static/charts/test2.json"));

    let datasets = use_state(|| pop_data.clone());
    let labels = use_state(|| pop_labels.clone());
    let opts = use_state(|| pop_opts.clone());
    let callback = {
        let datasets = datasets.clone();
        let opts = opts.clone();
        let labels = labels.clone();
        Callback::from(
            move |(data, data_lables, options): (Vec<Dataset>, Vec<JsValue>, JsValue)| {
                datasets.set(data);
                labels.set(data_lables);
                opts.set(options);
            },
        )
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

    {
        let quote_observer = quote_observer.clone();
        use_effect_once(move || {
            let elems = gloo::utils::document().get_elements_by_class_name("side-bar");
            for i in 0..elems.length() {
                let side_bar_children = elems.item(i).expect("should be in collection").children();
                for i in 0..side_bar_children.length() {
                    let elem = side_bar_children.item(i).expect("should be in collection");
                    quote_observer.observe(&elem)
                }
            }
            || ()
        });
    }

    html! {
        <>
            <div class="navbar">
                <h1>{ "The One Child Policy" }</h1>
                <span>{ "独生子女政策" }</span>
            </div>
            <h1><a href="#charts">{ "Chart" }</a></h1>
            <hr/>
            <div class={classes!("center", "media-small")} >
                <div class={classes!("side-by-side", "min-center")}>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>, JsValue)> text="Median Age" data={(pop_data, pop_labels, pop_opts)} callback={callback.clone()}/>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>, JsValue)> text="Fertility" data={(fertility_data, fertility_labels, fertility_opts)} {callback}/>
                </div>
                <Chart
                    id="main-chart"
                    datasets={(*datasets).clone()}
                    labels={(*labels).clone()}
                    opts={(*opts).clone()}
                />
            </div>
            <h1><a href="#timeline">{ "Timeline" }</a></h1>
            <hr/>
            <div class={classes!("side-by-side")}>
                <div class={classes!("side-bar")}>
                    <Quote text="Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat." top=10 alignment="end"/>
                    <Quote text="Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat." top=30 alignment="center"/>
                    <img src="./img/gathering.jpeg" alt="A gathering forms around a One Child Policy announcement" class={classes!("fit")} style="--top: 40%;"/>
                </div>
                <Cards
                    range={1980..2017}
                    year_data={
                        serde_json::from_str::<HashMap<u32, String>>(
                            include_str!("../static/year_data/year_data.json"))
                                .expect("should have valid json")
                    }
                />
                <div class={classes!("side-bar")}>
                    <img src="./img/one_child.jpeg" alt="A poster describing the ideal situation involving one child" class={classes!("fit")} style="--top: 20%;"/>
                    <img src="./img/one_child2.jpeg" alt="A poster saying that giving birth to one child is everyone's responsibility" class={classes!("fit")} style="--top: 60%;"/>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
