mod chart;
mod data_button;
mod dataset_loader;
mod topbar;

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
    html! {
        <>
            <TopBar title="The One Child Policy">
                <div class={classes!("flex", "flex-row", "flex-1", "gap-3")}>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>)> text="Population" data={(pop_data, pop_labels)} callback={callback.clone()}/>
                    <DataButton<(Vec<Dataset>, Vec<JsValue>)> text="Fertility" data={(fertility_data, fertility_labels)} {callback}/>
                </div>
                <Chart
                    id="mainChart"
                    datasets={(*datasets).clone()}
                    labels={(*labels).clone()}
                />
            </TopBar>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
