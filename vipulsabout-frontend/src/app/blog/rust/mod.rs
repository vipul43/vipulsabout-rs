use yew::prelude::*;

/// Rust component function
#[function_component(Rust)]
pub fn rust() -> Html {
    log::info!("Tech component");
    html! {
        <div>
            <h1>{ "Rust" }</h1>
        </div>
    }
}
