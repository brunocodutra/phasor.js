use super::Phasor;

use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(js_name = "closeTo")]
pub fn close_to(p: &Phasor, q: &Phasor, e: f64) -> bool {
    crate::assert::ulps_or_relative_eq(p, q, e)
}
