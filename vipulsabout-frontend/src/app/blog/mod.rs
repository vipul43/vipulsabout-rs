use yew::prelude::*;
use yew_router::prelude::*;

/// Rust component module
pub mod rust;

/// Tech component module
pub mod tech;

/// Not Found component module
pub mod not_found;

use crate::router::BlogRoute;

/// Blog component function
#[function_component(Blog)]
pub fn blog() -> Html {
    log::info!("Blog component");
    let navigator = use_navigator().expect("Failed to navigate from blog");
    let onclick_tech = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&BlogRoute::Tech))
    };
    let onclick_rust = Callback::from(move |_| navigator.push(&BlogRoute::Rust));

    html! {
        <div>
            <h1>{ "Blog" }</h1>
            <button onclick={onclick_tech}>{ "Tech" }</button>
            <button onclick={onclick_rust}>{ "Rust" }</button>
        </div>
    }
}
