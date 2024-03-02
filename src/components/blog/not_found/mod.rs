use yew::prelude::*;

/// Not found component function
#[function_component(BlogNotFound)]
pub fn blog_not_found() -> Html {
    html! {
        <div>
            <h1>{ "Blog Not Found" }</h1>
        </div>
    }
}
