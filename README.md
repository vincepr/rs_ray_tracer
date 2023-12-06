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

- building the wasm-project:
```
wasm_web/build/index.html
```