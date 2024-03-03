use std::time::Duration;
use wasm_bindgen_test::*;
use yew::platform::time::sleep;

// wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn test_async_pass() {
    sleep(Duration::new(5, 0)).await;
    assert_eq!(1, 1);
}
