mod cards;
mod chart;
mod data_button;
mod dataset_loader;
mod topbar;

use cards::Cards;
use chart::{Chart, Dataset};
use data_button::DataButton;
use dataset_loader::Chart as ChartJson;
use std::collections::HashMap;
use topbar::TopBar;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::HtmlElement;
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

    let top_bar_height = use_state_eq(|| 230);
    {
        let height = top_bar_height.clone();
        use_effect(move || {
            let top_bar = gloo::utils::document()
                .get_element_by_id("top-bar")
                .expect_throw("should have a TopBar")
                .dyn_into::<HtmlElement>()
                .expect_throw("TopBar should be in the DOM");
            height.set(top_bar.offset_height());
        });
    }

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
            <Cards
                id="cards"
                range={1980..2017}
                top_margin={-*top_bar_height}
                year_data={
                    serde_json::from_str::<HashMap<u32, String>>(
                        include_str!("../static/year_data/year_data.json"))
                            .expect("should have valid json")
                }
            />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
