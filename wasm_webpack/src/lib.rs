pub mod example8;

use wasm_bindgen::prelude::*;
use web_sys::console;

use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    console::log_1(&JsValue::from_str("wasm is hooked up and working."));
    Ok(())
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    width: u32,
    height: u32,
    _generator_string: &str,
) -> Result<(), JsValue> {
    let mut data = get_pixel_data(width, height);
    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    ctx.put_image_data(&data, 0.0, 0.0)
}

fn get_pixel_data(width: u32, height: u32) -> Vec<u8> {
    example8::build_example(width, height)
}
