use yew::prelude::*;

use gloo::console::log;

#[function_component(App)]
pub fn app() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    // let onclick_decrement = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter - 1;
    //         counter.set(value);
    //         // log!("Counter Decrement: ", counter);
    //     }
    // };

    log!("Counter", counter.to_string());
    html! {
        <div>
            <button { onclick }>{ "+1" }</button>
            // <button { onclick_decrement }>{ "-1" }</button>
            <p>{ *counter }</p>
        </div>
    }
}
