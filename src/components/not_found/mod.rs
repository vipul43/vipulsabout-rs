use yew::prelude::*;

/// Not found component function
#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div>
            <h1>{ "Not Found" }</h1>
        </div>
    }
}
