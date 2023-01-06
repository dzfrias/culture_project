mod chart;
mod dataset_loader;
mod topbar;

use chart::{Chart, Dataset};
use dataset_loader::Chart as ChartJson;
use topbar::TopBar;
use yew::{prelude::*, props};

#[function_component(App)]
fn app() -> Html {
    let chart: ChartJson = serde_json::from_str(include_str!("../static/charts/test.json"))
        .expect("Should have valid chart data");
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
    html! {
        <>
            <TopBar title="The One Child Policy">
                <p>{ "Hi" }</p>
                <Chart
                    id="mainChart"
                    datasets={datasets}
                    labels={labels}
                />
            </TopBar>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
