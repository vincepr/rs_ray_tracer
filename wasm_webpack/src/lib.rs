pub mod example8;

use wasm_bindgen::prelude::*;
use web_sys::console;

use wasm_bindgen::Clamped;
use web_sys::{CanvasRenderingContext2d, ImageData};

// when the wee_alloc feature flag is set, we use this as global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    console::log_1(&JsValue::from_str("wasm is hooked up and working."));
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct World {
  canvas_size: u32,
}
#[wasm_bindgen]
impl World {
  #[wasm_bindgen(constructor)]
  pub fn new(canvas_size: u32) -> Self {
    World {
      canvas_size,
    }
  }

  pub fn render(&self, y: f64) -> Result<ImageData, JsValue> {
    // Skip the "cost" of initializing the vector, as we are writing everywhere
    // in it later on
    let data_size = self.canvas_size as usize * 4;
    let mut data: Vec<u8> = Vec::with_capacity(data_size);
    unsafe {
      data.set_len(data_size);
    }
    
    //   #[allow(clippy::identity_op)]
    //   {
    //     data[(x * 4 + 0) as usize] = (fragment_color.red * 255.0).round() as u8;
    //     data[(x * 4 + 1) as usize] = (fragment_color.green * 255.0).round() as u8;
    //     data[(x * 4 + 2) as usize] = (fragment_color.blue * 255.0).round() as u8;
    //     data[(x * 4 + 3) as usize] = 255;
    //   }
    // }
    // let data = example8::build_example(self.canvas_size, self.canvas_size);
    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.canvas_size, 1)
  }
}

// #[wasm_bindgen]
// pub fn draw(
//     ctx: &CanvasRenderingContext2d,
//     width: u32,
//     height: u32,
//     _generator_string: &str,
// ) -> Result<(), JsValue> {
//     let data = get_pixel_data(width, height);
//     let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), width, height)?;
//     ctx.put_image_data(&data, 0.0, 0.0)
// }

fn get_pixel_data(width: u32, height: u32) -> Vec<u8> {
    example8::build_example(width, height)
}
