use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::{
    blog::{not_found::BlogNotFound, rust::Rust, tech::Tech, Blog},
    home::Home,
    not_found::NotFound,
};

/// Routes
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    /// Home route
    #[at("/")]
    Home,
    /// Blog route
    #[at("/blog")]
    Blog,
    /// Blog sub route
    #[at("/blog/*")]
    BlogRoute,
    /// Not found route
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Blog sub routes
#[derive(Clone, Routable, PartialEq)]
pub enum BlogRoute {
    /// Rust blog route
    #[at("/blog/rust")]
    Rust,
    /// Tech blog route
    #[at("/blog/tech")]
    Tech,
    /// Blog not found route
    #[not_found]
    #[at("/blog/404")]
    BlogNotFound,
}

/// Route switch
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Blog => html! {
            <Blog />
        },
        Route::BlogRoute => html! {
            <Switch<BlogRoute> render={switch_blog} />
        },
        Route::NotFound => html! {
            <NotFound />
        },
    }
}

/// Blog route switch
pub fn switch_blog(routes: BlogRoute) -> Html {
    match routes {
        BlogRoute::Rust => html! {
            <Rust />
        },
        BlogRoute::Tech => html! {
            <Tech />
        },
        BlogRoute::BlogNotFound => html! {
            <BlogNotFound />
        },
    }
}
