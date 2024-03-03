use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_pass() {
    let x = 1;
    assert_eq!(1, x);
}
