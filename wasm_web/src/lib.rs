use mathlib::visual::{canvas::Canvas, color::Col};
use photon_rs::open_image;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Rust compiled and hooked into the js console.log()".into());
    manually_create_gradient_file();
    insert_picture("canvas");
}

pub fn manually_create_gradient_file() {
    let mut canvas = Canvas::new(100, 200);
    for (i, row) in canvas.arr.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = Col::new(i as f32 / 100.0, 0.8, j as f32 / 200.0);
            // console::log_1(&(col.to_string()).into());
        }
    }
    console::log_1(&(canvas.canvas_to_ppm().to_string()).into());
    // write_to_file("./out.ppm", canvas.canvas_to_ppm());
}

// takes DOM id of a canvas element and puts our image in there:
#[wasm_bindgen]
pub fn insert_picture(element_id: &str) {
    // let window = web_sys::window().expect("no global 'window' found");
    // let document = window.document().expect("expect document on window");
    // let canvas = document.get_element_by_id(element_id)
    //     .expect("no canvas found");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document
        .get_element_by_id(element_id)
        .expect("no canvas found");

    let canvas_html = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

    // let canvas: web_sys::HtmlCanvasElement = canvas.
    // .dyn_into::<web_sys::HtmlCanvasElement>()
    // .map_err(|_| ())
    // .unwrap();

    let context = canvas_html
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // we create a mutable ref so we can modify it later:
    let mut image = open_image(canvas_html.clone(), context.clone());
    for (pix) in image.get_raw_pixels().iter_mut() {
        *pix = 150;
    }
    photon_rs::channels::alter_red_channel(&mut image, 40);
    photon_rs::putImageData(canvas_html, context, image);
}use mathlib::visual::{canvas::Canvas, color::Col};
use photon_rs::open_image;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::console;


#[wasm_bindgen(start)]
pub fn main() {
    console::log_1(&"Rust compiled and hooked into the js console.log()".into());
    manually_create_gradient_file();
    insert_picture("canvas");
}

pub fn manually_create_gradient_file() {
    let mut canvas = Canvas::new(100, 200);
    for (i, row) in canvas.arr.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            *col = Col::new(i as f32 / 100.0, 0.8, j as f32 / 200.0);
            // console::log_1(&(col.to_string()).into());
        }
    }
    console::log_1(&(canvas.canvas_to_ppm().to_string()).into());
    // write_to_file("./out.ppm", canvas.canvas_to_ppm());
}

// takes DOM id of a canvas element and puts our image in there:
#[wasm_bindgen]
pub fn insert_picture(element_id: &str) {
    // let window = web_sys::window().expect("no global 'window' found");
    // let document = window.document().expect("expect document on window");
    // let canvas = document.get_element_by_id(element_id)
    //     .expect("no canvas found");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let canvas = document
        .get_element_by_id(element_id)
        .expect("no canvas found");

    let canvas_html = canvas.dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    // let canvas: web_sys::HtmlCanvasElement = canvas.
        // .dyn_into::<web_sys::HtmlCanvasElement>()
        // .map_err(|_| ())
        // .unwrap();

    let context = canvas_html
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // we create a mutable ref so we can modify it later:
    let mut image = open_image(canvas_html.clone(), context.clone());
    for pix in image.get_raw_pixels().iter_mut() {
        *pix = 150;
    }
    photon_rs::channels::alter_red_channel(&mut image, 40);
    photon_rs::putImageData(canvas_html, context, image);
}
