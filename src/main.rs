mod chart;
mod topbar;

use chart::ChartComponent;
use topbar::TopBar;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <TopBar title="The One Child Policy">
                <p>{ "Hi" }</p>
                <ChartComponent id="mainChart"/>
            </TopBar>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
