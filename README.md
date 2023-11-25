# Ray Tracer Challenge - writing a 3D renderer

Along the Book **The Ray Tracer Challenge** by Jamis Buck, the goal is to write a 3D Renderer in rust. And play arround with Flamegraph, Wasm, Wasm combined with Webworkers and whatever else comes up.

- Goals are using as few as possible dependencies (for the core functionality )

## Whitted ray tracing - aka recursive ray tracing
Ot works by recursively spawning rays and bouncing them arround the scene, to calculate what color each pixel should have.
1. Cast a ray into the scene and find the surface it hits
2. Cast a ray from that point to each light source. This determines what lights illuminate this point.
3. If the surface is reflective, cast a new ray in the direction of the reflection. To recursively dertermine the color reflected.
4. If the surface is transparent, we do the same in direction of the refraction.
5. Combine all colors that contribute to a point (surface & reflection & refraction)


```rs
fn main() {
    println!("Hello, world!");
    let t = Tupl::new_point(1.0, 2.0, 3.0);
    println!("t is currently: {t:?}");
}
/// (x,y,z,0) - vector
/// (x,y,z,1) - point
#[derive(Debug)]
pub struct Tupl{
    x: f32,
    y: f32,
    z: f32,
    w: isize,
}
impl Tupl {
    pub fn new(x: f32,y: f32, z:f32, w: isize) -> Self {
        Tupl { x, y, z, w }
    }

    pub fn new_vector(x: f32, y: f32, z:f32) -> Self {
        Tupl::new(x, y, z, 0)
    }

    pub fn new_point(x: f32, y: f32, z:f32) -> Self {
        Tupl::new(x, y, z, 1)
    }
}
```