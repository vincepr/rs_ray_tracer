use crate::yaml_parsing;

pub fn parse_yaml() {
    // read args
    let args: Vec<String> = std::env::args().collect();

    // default value if no args to file.
    let path = if args.len() != 2 {
        let default = "";
        println!("did not provide path to '.yaml' file...");
        println!("trying default path: {default}");
        println!("example: 'cargo run -- ./samples/yaml_samples/book_cover.yaml'");
        "./samples/yaml_samples/book_cover.yaml".to_string()
    } else {
        args[1].clone()
    };

    // read the file
    println!("rendering file: {path}");
    let yaml_str = std::fs::read_to_string(path).unwrap();

    // parse yaml to scene
    let scene = yaml_parsing::yaml::parse_str(yaml_str);

    // render scene to output .ppm and .png files in parallel
    scene.run_scene()
}
