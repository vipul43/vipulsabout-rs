use yew::prelude::*;
use yew_router::prelude::*;

/// Blog page
pub mod blog;

/// Home page
pub mod home;

/// Not Found page
pub mod not_found;

use crate::components::navbar::NavBar;
use crate::router::{switch, Route};

/// Application component function
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <NavBar />
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}
