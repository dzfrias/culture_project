use gloo::timers::callback::Timeout;
use serde::{ser::SerializeStruct, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/js/lineChart.js")]
extern "C" {
    pub type LineChart;

    #[wasm_bindgen(constructor)]
    pub fn new(labels: Vec<JsValue>, datasets: Vec<JsValue>, opts: JsValue) -> LineChart;

    #[wasm_bindgen(method)]
    pub fn draw(this: &LineChart, element_id: &str);

    #[wasm_bindgen(method, js_name = newConfig)]
    pub fn new_config(
        this: &LineChart,
        labels: Vec<JsValue>,
        datasets: Vec<JsValue>,
        opts: JsValue,
    );
}

#[derive(Debug, PartialEq, Properties, Clone)]
pub struct Dataset {
    pub label: AttrValue,
    pub data: Vec<f32>,
    #[prop_or("rgb(255, 255, 255)".into())]
    pub bg_color: AttrValue,
    #[prop_or("rgb(255, 255, 255)".into())]
    pub border_color: AttrValue,
}

impl Serialize for Dataset {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Dataset", 4)?;
        state.serialize_field("label", &self.label.as_str())?;
        state.serialize_field("data", &self.data)?;
        state.serialize_field("backgroundColor", &self.bg_color.as_str())?;
        state.serialize_field("borderColor", &self.border_color.as_str())?;
        state.end()
    }
}

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
    pub datasets: Vec<Dataset>,
    pub labels: Vec<JsValue>,
    pub opts: JsValue,
}

pub enum Msg {
    Draw,
}

pub struct Chart {
    chart: LineChart,
    #[allow(dead_code)]
    draw_timer: Timeout,
}

impl Component for Chart {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        let stand_alone_timer = {
            let link = link.clone();
            Timeout::new(10, move || link.send_message(Msg::Draw))
        };
        let props = ctx.props();
        Self {
            chart: LineChart::new(
                props.labels.clone(),
                props
                    .datasets
                    .iter()
                    .map(|dataset| serde_wasm_bindgen::to_value(dataset).unwrap())
                    .collect(),
                props.opts.clone(),
            ),
            draw_timer: stand_alone_timer,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Draw => {
                self.chart.draw(&ctx.props().id);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class={classes!("chart")}>
                <canvas id={&ctx.props().id}></canvas>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            return;
        }
        let props = ctx.props();
        self.chart.new_config(
            props.labels.clone(),
            props
                .datasets
                .iter()
                .map(|dataset| serde_wasm_bindgen::to_value(dataset).unwrap())
                .collect(),
            props.opts.clone(),
        );
    }
}
