use std::collections::HashMap;

use mathlib::{
    io::ppm::write_to_file,
    mathstructs::{matrix::Matrix, point::Point, vector::Vector},
    object::{cube::Cube, plane::Plane, sphere::Sphere, Object},
    visual::{
        camera::Camera,
        color::{Col, WHITE},
        light::Light,
        material::Material,
        world::World,
    },
};
use yaml_rust2::{yaml, Yaml, YamlLoader};

use crate::png_io::canvas_png_save;

pub struct SceneToRun {
    pub camera: Option<Camera>,
    pub lights: Vec<Light>,
    pub objects: Vec<Object>,
}

impl SceneToRun {
    pub fn new() -> Self {
        Self {
            camera: None,
            lights: vec![],
            objects: vec![],
        }
    }

    pub fn run_scene(self) {
        let mut world = World::new();
        for light in self.lights {
            world.lights.push(light);
        }
        for obj in self.objects {
            world.objects.push(obj)
        }

        let canvas = crate::parallel::render_parallel(self.camera.expect("unreachable."), world);
        write_to_file("./out.ppm", canvas.canvas_to_ppm());
        canvas_png_save(&canvas, "./out.png");
    }
}

pub fn parse_str(yaml_str: String) -> SceneToRun {
    let docs = YamlLoader::load_from_str(&yaml_str).expect("Unable to load yaml from string.");
    let root_nodes = docs[0]
        .as_vec()
        .expect("Bad yaml structure. Expected multiple entries.");

    let defs = parse_definitions(root_nodes);

    let mut camera: Option<Camera> = None;
    let mut lights: Vec<Light> = vec![];
    let mut objects: Vec<Object> = vec![];

    for node in root_nodes.iter().map(|yaml| yaml.as_hash().unwrap()) {
        if let Some(add_node) = node.get(&Yaml::from_str("add")) {
            match add_node.as_str().unwrap() {
                "camera" => camera = Some(camera_from_node(node)),
                "light" => lights.push(light_from_node(node)),
                typ if typ == "cube" || typ == "plane" || typ == "sphere" => {
                    objects.push(obj_from_node(node, typ, &defs))
                }
                _ => unimplemented!("missing support for type {}", add_node.as_str().unwrap()),
            }
        }
    }

    if camera.is_none() {
        panic!("Expect camera to be 'add'-ed.");
    }
    if lights.len() < 1 {
        panic!("Expect at least one light to be 'add'-ed.");
    }
    // dbg!(&objects);
    SceneToRun {
        camera,
        lights,
        objects,
    }
}

fn obj_from_node(node: &yaml::Hash, type_name: &str, defs: &Definitions) -> Object {
    let mut object = match type_name {
        "cube" => Cube::new(),
        "plane" => Plane::new(),
        "sphere" => Sphere::new(),
        _ => {
            unimplemented!()
        }
    };
    object.material = material_from_node(defs, node);
    object.transformation = transform_from_node(defs, node).unwrap();

    object
}

// ...
// transform:
// - [ rotate-x, 1.5707963267948966 ] # pi/2
// - [ translate, 0, 0, 500 ]
//
// ...
// transform:
// - large-obj                  #-definined-above
// - [ translate, 0, 0, 500 ]   # make changes to large-obj
fn transform_from_node(defs: &Definitions, node: &yaml::Hash) -> Option<Matrix> {
    let mut matrix = Matrix::default(); // identity matrix that doesn't transforms

    // we check for transformations if we find the keyword. We expect them to be an array.
    if let Some(transform_node) = node.get(&Yaml::from_str("transform")) {
        let transforms_raw = transform_node
            .as_vec()
            .expect("expect 'transform:' to contain an array.");
        let mut transforms_resolved = vec![];
        recursive_resolve_transforms(defs, transforms_raw, &mut transforms_resolved);

        for one_transform in transforms_resolved {
            let arr = array_from_yaml(defs, &one_transform);

            // expect identifier [0] that describes 'transformation-type'
            let identifier = arr[0]
                .as_str()
                .expect("expect each transformation to start with a type.");

            //expect next numbers to be the 'args'
            matrix = match identifier {
                "rotate-x" => matrix.rotate_x(as_f64(&arr[1])?),
                "rotate-y" => matrix.rotate_y(as_f64(&arr[1])?),
                "rotate-z" => matrix.rotate_z(as_f64(&arr[1])?),
                "translate" => matrix.translate(as_f64(&arr[1])?, as_f64(&arr[2])?, as_f64(&arr[3])?),
                "scale" => matrix.scale(as_f64(&arr[1])?, as_f64(&arr[2])?, as_f64(&arr[3])?),
                "shear" => matrix.shear(
                    as_f64(&arr[1])?,
                    as_f64(&arr[2])?,
                    as_f64(&arr[3])?,
                    as_f64(&arr[4])?,
                    as_f64(&arr[5])?,
                    as_f64(&arr[6])?,
                ),
                unexp => panic!("Unexpected transformation identifier : {:?}, ", unexp),
            };
        }
    }

    return Some(matrix);
}

/// push resolved transformations into transformations vec. Recurses trough definitions.
fn recursive_resolve_transforms(
    defs: &Definitions,
    array: &[Yaml],
    transformations: &mut Vec<Yaml>,
) {
    for transform in array {
        match transform[0].as_str() {
            Some(_) => transformations.push(transform.clone()),
            None => {
                let embedded_transformations = array_from_yaml(defs, transform);
                recursive_resolve_transforms(defs, embedded_transformations, transformations);
            }
        }
    }
}

fn array_from_yaml<'a>(definitions: &'a Definitions, yaml: &'a Yaml) -> &'a yaml::Array {
    match yaml.as_vec() {
        Some(hash) => hash,
        None => definitions
            .get(yaml)
            .unwrap_or_else(|| panic!("Definition {:?} not found", yaml))
            .as_vec()
            .unwrap(),
    }
}

fn material_from_node(defs: &Definitions, node: &yaml::Hash) -> Material {
    let mut default = Material::default();
    match node.get(&Yaml::from_str("material")) {
        Some(material_node) => {
            let mat_hash = &find_hash_in_definitions(defs, material_node);
            if let Some(val) = f64_from_key(mat_hash, "ambient") {
                default.ambient = val;
            }
            if let Some(val) = f64_from_key(mat_hash, "diffuse") {
                default.diffuse = val;
            }
            if let Some(val) = f64_from_key(mat_hash, "reflective") {
                default.reflective = val;
            }
            if let Some(val) = f64_from_key(mat_hash, "refractive-index") {
                default.refractive_index = val;
            }
            if let Some(val) = f64_from_key(mat_hash, "shininess") {
                default.shininess = val;
            }
            if let Some(val) = f64_from_key(mat_hash, "specular") {
                default.specular = val;
            }
            if let Some(val) = f64_from_key(mat_hash, "transparency") {
                default.transparency = val;
            }
            if let Some(val) = color_from_key(&mat_hash, "color") {
                default.color(val);
            }
            // TODO: extend for pattern if i care to continute.

            default
        }
        None => default,
    }
}

fn light_from_node(node: &yaml::Hash) -> Light {
    Light::new_point_light(
        point_from_key(node, "at").unwrap_or(Point::new(-6., 6., -10.)),
        color_from_key(node, "intensity").unwrap_or(WHITE),
    )
}

fn camera_from_node(node: &yaml::Hash) -> Camera {
    Camera::new(
        usize_from_key(node, "width").unwrap_or(200),
        usize_from_key(node, "height").unwrap_or(100),
        f64_from_key(node, "field-of-view").unwrap_or(std::f64::consts::PI / 4.),
    )
    .with_transform(Matrix::view_transform_new(
        point_from_key(node, "from").unwrap_or(Point::new(-6., 6., -10.)),
        point_from_key(node, "to").unwrap_or(Point::new(6., 0., 6.)),
        vector_from_key(node, "up").unwrap_or(Vector::new(-0.45, 1., 0.)),
    ))
}

/// maps both float and int to -> f64
fn as_f64(yaml: &Yaml) -> Option<f64> {
    if let Some(float) = yaml.as_f64() {
        return Some(float);
    }
    if let Some(int) = yaml.as_i64() {
        return Some(int as f64);
    }
    None
}

fn three_f64(node: &yaml::Hash, key: &str) -> Option<(f64, f64, f64)> {
    let xyz = node.get(&Yaml::from_str(key))?.as_vec()?;
    assert!(xyz.len() == 3, "Chooridantes must always have 3 values.");
    Some((as_f64(&xyz[0])?, as_f64(&xyz[1])?, as_f64(&xyz[2])?))
}

fn color_from_key(node: &yaml::Hash, key: &str) -> Option<Col> {
    let xyz = three_f64(node, key)?;
    assert!(xyz.0 >= 0. && xyz.0 <= 1.);
    assert!(xyz.1 >= 0. && xyz.1 <= 1.);
    assert!(xyz.2 >= 0. && xyz.2 <= 1.);
    Some(Col::new(xyz.0, xyz.1, xyz.2))
}

fn point_from_key(node: &yaml::Hash, key: &str) -> Option<Point> {
    let xyz = three_f64(node, key)?;
    Some(Point::new(xyz.0, xyz.1, xyz.2))
}

fn vector_from_key(node: &yaml::Hash, key: &str) -> Option<Vector> {
    let xyz = three_f64(node, key)?;
    Some(Vector::new(xyz.0, xyz.1, xyz.2))
}

fn f64_from_key(node: &yaml::Hash, key: &str) -> Option<f64> {
    as_f64(node.get(&Yaml::from_str(key))?)
}

fn usize_from_key(node: &yaml::Hash, key: &str) -> Option<usize> {
    let int = node.get(&Yaml::from_str(key))?.as_i64()?;
    if int < 0 {
        return None;
    }
    Some(int as usize)
}

type Definitions<'a> = HashMap<&'a Yaml, Yaml>;

fn parse_definitions(yaml_list: &Vec<Yaml>) -> Definitions {
    let mut definitions = HashMap::new();

    for node in yaml_list.iter().map(|yaml| yaml.as_hash().unwrap()) {
        if let Some(define_node) = node.get(&Yaml::from_str("define")) {
            let this_values = node
                .get(&Yaml::from_str("value"))
                .expect("Could not find values in Definition.");

            // if we find 'extend' we combine parent & self (only 1 level of inheritance possible)
            let combined_values = match node.get(&Yaml::from_str("extend")) {
                // use only itself
                None => this_values.clone(),
                // combine with inherited parent-values
                Some(extend_id) => {
                    if let Some(self_values_hash) = this_values.as_hash() {
                        let mut parent_hash =
                            find_hash_in_definitions(&definitions, extend_id).clone();
                        parent_hash.extend(self_values_hash.clone().into_iter());
                        Yaml::Hash(parent_hash)
                    } else {
                        unreachable!("Not implemented. ");
                    }
                }
            };
            definitions.insert(define_node, combined_values);
        }
    }
    definitions
}

/// to search pre-defined 'variable-like' / 'global-variables' definitions with the 'define'-keyword
/// these can be used while adding other elements/definitions to the scene.
/// fn get_hash<'a>(definitions: &'a Definitions, yaml: &'a Yaml) -> &'a yaml::Hash {
fn find_hash_in_definitions<'a>(defs: &'a Definitions, yaml_key: &'a Yaml) -> &'a yaml::Hash {
    match yaml_key.as_hash() {
        Some(hash) => hash,
        None => defs
            .get(yaml_key)
            .unwrap_or_else(|| {
                panic!(
                    "Definition {:?} not found, must be declared before use.",
                    yaml_key
                )
            })
            .as_hash()
            .unwrap(),
    }
}
