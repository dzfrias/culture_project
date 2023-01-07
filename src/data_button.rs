use yew::prelude::*;

#[derive(Properties, PartialEq, Debug, Clone)]
pub struct Props<T>
where
    T: PartialEq,
{
    pub text: AttrValue,
    pub callback: Callback<T>,
    pub data: T,
}

#[function_component(DataButton)]
pub fn data_button<T: PartialEq + Clone + 'static>(props: &Props<T>) -> Html {
    let onclick = {
        let props = props.clone();
        Callback::from(move |_| {
            props.callback.emit(props.data.clone());
        })
    };
    html! {
        <button {onclick}>{ &props.text }</button>
    }
}
