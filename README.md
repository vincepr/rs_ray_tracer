# Ray Tracer Challenge - writing a 3D renderer

- live example https://vincepr.github.io/rs_ray_tracer/

## Example render of the book-cover
- generate the book cover fromt the parsed yaml file `.data/yaml_samples/book_cover.yaml` 

![book_cover_render](data/yaml_samples/appendix1_book_cover.png)

- chapter 7 - the first decent render

![book_cover_render](data/chapter7.png)

- chapter 8 - added shadows

![book_cover_render](data/chapter8.png)

- chapter 11 - added reflection and refraction

![book_cover_render](data/chapter11.png)

## The Ray Tracer Challange
Along the Book **The Ray Tracer Challenge** by Jamis Buck, the goal is to write a 3D Renderer in rust. And play arround with Flamegraph, Wasm, Wasm combined with Webworkers and whatever else comes up.

- Goals are using as few as possible dependencies (for the core functionality )

## Whitted ray tracing - aka recursive ray tracing
Ot works by recursively spawning rays and bouncing them arround the scene, to calculate what color each pixel should have.
1. Cast a ray into the scene and find the surface it hits
2. Cast a ray from that point to each light source. This determines what lights illuminate this point.
3. If the surface is reflective, cast a new ray in the direction of the reflection. To recursively dertermine the color reflected.
4. If the surface is transparent, we do the same in direction of the refraction.
5. Combine all colors that contribute to a point (surface & reflection & refraction)

## Flamegraph and perf for performance-testing
- perf in wsl
```
apt install linux-tools-generic
cargo install flamegraph

PERF=/usr/lib/linux-tools/5.15.0-91-generic/perf cargo flamegraph -o flamegraph.svg

sudo /usr/lib/linux-tools/5.15.0-91-generic/perf record -g --call-graph dwarf ./unix_compiled
```
## How to build / run
- for the wasm-javascript-webpage check out `./wasm_webpack/README.md`
- for the cli-tool. With rust and cargo installed you can just run: `cargo run --release -- ./samples/yaml_samples/book_cover.yaml`
