use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

/// Home component function
#[function_component(Home)]
pub fn home() -> Html {
    log::info!("Home component");
    let navigator = use_navigator().expect("Failed to navigate from home");
    let onclick_home = {
        let navigator = navigator.clone();
        Callback::from(move |_| navigator.push(&Route::Home))
    };
    let onclick_blog = Callback::from(move |_| navigator.push(&Route::Blog));

    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button onclick={onclick_home}>{ "Home" }</button>
            <button onclick={onclick_blog}>{ "Blog" }</button>
        </div>
    }
}
