use gloo::timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(module = "/js/lineChart.js")]
extern "C" {
    pub type LineChart;

    #[wasm_bindgen(constructor)]
    pub fn new(labels: Vec<JsValue>) -> LineChart;

    #[wasm_bindgen(method)]
    pub fn draw(this: &LineChart, element_id: &str);
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Dataset {
    label: String,
    data: Vec<JsValue>,
}

#[derive(Debug, Properties, PartialEq)]
pub struct Props {
    pub id: AttrValue,
}

pub enum Msg {
    Draw,
}

pub struct ChartComponent {
    pub chart: LineChart,
    pub draw_timer: Timeout,
}

impl Component for ChartComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link();
        let stand_alone_timer = {
            let link = link.clone();
            Timeout::new(10, move || link.send_message(Msg::Draw))
        };
        Self {
            chart: LineChart::new((1..=6).map(|num| num.into()).collect()),
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
            <div>
                <canvas id={&ctx.props().id} width="600" height="500"></canvas>
            </div>
        }
    }
}
