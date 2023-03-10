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
    let (age_data, age_labels, age_opts) =
        get_datasets(include_str!("../static/charts/median_age.json"));
    let (fertility_data, fertility_labels, fertility_opts) =
        get_datasets(include_str!("../static/charts/fertility.json"));
    let (pop_data, pop_labels, pop_opts) =
        get_datasets(include_str!("../static/charts/rate_of_pop_change.json"));
    let (planning_data, planning_labels, planning_opts) =
        get_datasets(include_str!("../static/charts/family_planning_demand.json"));

    let datasets = use_state(|| age_data.clone());
    let labels = use_state(|| age_labels.clone());
    let opts = use_state(|| age_opts.clone());
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
        opts.root_margin("-53% 0% -10% 0%");
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
                <span>{ "??????????????????" }</span>
            </div>
            <h1><a href="#charts">{ "Chart" }</a></h1>
            <hr/>
            <div class={classes!("center", "media-small")} >
                <div class={classes!("side-by-side", "min-center")}>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>, JsValue)> text="Median Age" data={(age_data, age_labels, age_opts)} callback={&callback}/>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>, JsValue)> text="Fertility" data={(fertility_data, fertility_labels, fertility_opts)} callback={&callback}/>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>, JsValue)> text="Family Planning" data={(planning_data, planning_labels, planning_opts)} callback={&callback}/>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>, JsValue)> text="Population Change" data={(pop_data, pop_labels, pop_opts)} {callback}/>
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
                    <Quote text="The cost of bringing up two kids would kill us!" top=10 alignment="end"/>
                    <Quote text="But who wants to have three kids? Young people could have two kids at most. The fundamental issue is living costs are too high and life pressures are too huge." top=30 alignment="center"/>
                    <img src="./img/gathering.jpeg" alt="A gathering forms around a One Child Policy announcement" class={classes!("fit")} style="--top: 40%;"/>
                    <p style="--top: 53%; --alignment: center;" class={classes!("small-box")}>{ "Even though the One-Child Policy started with the hope the economy would improve, there is some debate if it will even end up harming the economy." }</p>
                    <img src="./img/one_child3.jpg" alt="A poster of a mother carrying one child on her arm" style="--top: 70%" class={classes!("fit")}/>
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
                    <p style="--top: 35%; --alignment: start;" class={classes!("small-box")}>{ "The Chinese public feared having more than one child, a fear that was based in Malthusian beliefs.\nMalthusianism is the idea that population grows exponentially as supplies grow linearly." }</p>
                    <Quote text="From the first day of my aunt's pregnancy, she and my uncle had to hide in order to avoid getting caught by the officials from the local family planning unit" top=45 alignment="center"/>
                    <img src="./img/one_child2.jpeg" alt="A poster saying that giving birth to one child is everyone's responsibility" class={classes!("fit")} style="--top: 60%;"/>
                    <p class={classes!("small-box")} style="--top: 80%; --alignment: center;">{ "Due to the conditions of the policy, many couples killed or aborted their female babies." }</p>
                </div>
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
