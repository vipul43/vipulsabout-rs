use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    blog::{not_found::BlogNotFound, rust::Rust, tech::Tech, Blog},
    home::Home,
    not_found::NotFound,
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog")]
    Blog,
    #[at("/blog/*")]
    BlogRoute,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq)]
pub enum BlogRoute {
    #[at("/blog/rust")]
    Rust,
    #[at("/blog/tech")]
    Tech,
    #[not_found]
    #[at("/blog/404")]
    BlogNotFound,
}

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
