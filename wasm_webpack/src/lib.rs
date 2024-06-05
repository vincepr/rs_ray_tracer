
use mathlib_renderer::visual::color;
use parselib_yaml::yaml::SceneToRun;
use wasm_bindgen::prelude::*;
use web_sys::console;

use wasm_bindgen::Clamped;
use web_sys::ImageData;

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
    // console::log_1(&JsValue::from_str("wasm is hooked up and working."));
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub struct WasmRenderer {
  scene: SceneToRun,
  pub height: u32,
  pub width: u32,
}
#[wasm_bindgen]
impl WasmRenderer {
  #[wasm_bindgen(constructor)]
  pub fn new(yaml_str: &str) -> Self {
    let scene = SceneToRun::new_from_yaml(yaml_str);
    let height = scene.camera.height as u32;
    let width = scene.camera.width as u32;
    Self { scene, height, width }
  }

  fn get_row_pixels(&self, y: usize) -> Vec<u8> {
    let mut raw_pixels: Vec<u8> = 
      Vec::with_capacity(self.width as usize * 4);

    for x in 0..self.scene.camera.width {
      let ray = self.scene.camera.ray_for_pixel(x, y);
      let color = self.scene.world.color_at(&ray, 4);
      raw_pixels.push(color::base_255(color.r));
      raw_pixels.push(color::base_255(color.g));
      raw_pixels.push(color::base_255(color.b));
      raw_pixels.push(255); // transparency
    }

    raw_pixels
  }

  pub fn row_to_image_pixels(&self, y:usize) -> Result<ImageData, JsValue> {
    let data = self.get_row_pixels(y);
    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.width, 1)
  }

  // /// directly draws to the Javascript-canvas element's memory
  // pub fn draw_row_to_canvas(
  //   &self,
  //   ctx: &CanvasRenderingContext2d,
  //   y: usize,
  // ) -> Result<(), JsValue> {
  //   let data = self.get_row_pixels(y);
  //   let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&data), self.width, 1)?;
  //   ctx.put_image_data(&data, 0.0, 0.0)
  // }
}
