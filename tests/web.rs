//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

use phasor::fourty_two;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(fourty_two(), 42);
}
