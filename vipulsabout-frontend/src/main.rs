//! Main

use vipulsabout_frontend::app::App;

fn main() {
    log::info!("Initializing logging");
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Successfully initialized logging");

    yew::Renderer::<App>::new().render();
}
