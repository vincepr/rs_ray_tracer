use mathlib_renderer::io::ppm::write_to_file;
use parselib_yaml::yaml::SceneToRun;

use crate::png_io::canvas_png_save;

pub fn parse_yaml_cli() {
    // read args
    let args: Vec<String> = std::env::args().collect();

    // default value if no args to file.
    let path = if args.len() != 2 {
        let default = "./data/yaml_samples/book_cover.yaml";
        println!("did not provide path to '.yaml' file...");
        println!("No path was provided. Trying default path: {default}");
        println!("like: 'cargo run --release -- ./samples/yaml_samples/book_cover.yaml'");
        "./data/yaml_samples/book_cover.yaml".to_string()
    } else {
        args[1].clone()
    };

    // read the file
    println!("rendering file: {path}");
    let yaml_str = std::fs::read_to_string(path).unwrap();

    // parse yaml to scene
    let scene = SceneToRun::new_from_yaml(&yaml_str);

    // render scene to output .ppm and .png files in parallel
    run_scene(scene);
}

/// render the scene out and write the result to a file
pub fn run_scene(scene: SceneToRun) {

    let canvas = crate::parallel::render_parallel(scene.camera, scene.world);
    write_to_file("./out.ppm", canvas.canvas_to_ppm());
    canvas_png_save(&canvas, "./out.png");
}
