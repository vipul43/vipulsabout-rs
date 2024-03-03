//! Navigation Bar

use yew::prelude::*;

/// NavBar component function
#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    log::info!("Navbar component");

    html! {
        <nav>
            <div class="navbar-container">
                <div class="logo">
                    <a href="#">{ "Logo" }</a>
                </div>
                <ul class="navbar-menu">
                    <li><a href="#">{ "Home" }</a></li>
                    <li><a href="#">{ "About" }</a></li>
                    <li><a href="#">{ "Services" }</a></li>
                    <li><a href="#">{ "Contact" }</a></li>
                </ul>
            </div>
        </nav>

    }
}
