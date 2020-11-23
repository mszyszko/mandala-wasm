// Use `wee_alloc` as the global allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;

mod mandala;
mod point;
mod segment;

use mandala::Mandala;

#[wasm_bindgen]
pub fn generate_mandala(
    seed: &str,
    nr_of_segments: u32,
    detail: u32,
    radius: f32,
    color: String,
) -> String {
    let mandala = Mandala::new(seed, nr_of_segments, radius, detail);
    mandala.to_svg(2.0 * radius, 2.0 * radius, &color)
}
