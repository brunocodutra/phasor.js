use super::{AbsDiffEq, Phasor, UlpsEq};
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(js_name = "closeTo")]
pub fn close_to(p: &Phasor, q: &Phasor, e: Option<f64>, ulps: Option<f64>) -> bool {
    let e = e.unwrap_or_else(Phasor::default_epsilon);
    let ulps = ulps
        .map(|u| u.max(0f64).min(u32::MAX as f64) as u32)
        .unwrap_or_else(Phasor::default_max_ulps);

    p.ulps_eq(&q, e, ulps)
}
