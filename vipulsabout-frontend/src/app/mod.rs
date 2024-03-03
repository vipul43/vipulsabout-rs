use yew::prelude::*;
use yew_router::prelude::*;

pub mod blog;
pub mod home;
pub mod not_found;

use crate::router::{switch, Route};

/// Application component function
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
