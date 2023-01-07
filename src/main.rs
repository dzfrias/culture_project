mod chart;
mod data_button;
mod dataset_loader;
mod topbar;

use std::collections::HashMap;

use chart::{Chart, Dataset};
use data_button::DataButton;
use dataset_loader::Chart as ChartJson;
use topbar::TopBar;
use wasm_bindgen::JsValue;
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

    let year_data: HashMap<i32, &str> =
        serde_json::from_str(include_str!("../static/year_data/year_data.json"))
            .expect("should have valid data");
    let cards = (1980..2017).map(|year| {
        let event = year_data.get(&year).unwrap_or(&"");
        html! {
            if event.is_empty() {
                <div class={classes!("border-white", "border", "p-2")}>
                    <p class={classes!("text-white")}>{ year }</p>
                </div>
            } else {
                <div class={classes!("border-white", "border", "h-[30vh]", "w-[80vw]", "md:w-[50vw]", "flex", "justify-center", "items-center", "flex-col", "p-5")}>
                    <p class={classes!("text-white", "flex-auto", "text-xl")}>{ year }</p>
                    <p class={classes!("text-white", "flex-[3_3_0%]", "text-center")}>{ event }</p>
                </div>
            }
        }
    });

    html! {
        <>
            <TopBar title="The One Child Policy">
                <div class={classes!("md:flex", "md:flex-row", "md:gap-4")}>
                    <div class={classes!("flex", "md:flex-col", "flex-1", "gap-3", "justify-center")}>
                        <DataButton<(Vec<Dataset>, Vec<JsValue>)> text="Population" data={(pop_data, pop_labels)} callback={callback.clone()}/>
                        <DataButton<(Vec<Dataset>, Vec<JsValue>)> text="Fertility" data={(fertility_data, fertility_labels)} {callback}/>
                    </div>
                    <Chart
                        id="mainChart"
                        datasets={(*datasets).clone()}
                        labels={(*labels).clone()}
                    />
                </div>
            </TopBar>
            <div class={classes!("flex", "flex-col", "items-center", "gap-10", "my-8")}>
                { for cards }
            </div>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
