use yew::prelude::*;

/// Tech component function
#[function_component(Tech)]
pub fn tech() -> Html {
    log::info!("Tech component");
    html! {
        <div>
            <h1>{ "Tech" }</h1>
        </div>
    }
}
