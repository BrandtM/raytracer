# Raytracer in Rust

This raytracer was written in Rust and is based on [Peter Shirley's "Ray Tracing in One Weekend"](http://www.realtimerendering.com/raytracing/Ray%20Tracing%20in%20a%20Weekend.pdf).  
This version was rewritten using the awesome [rayon library](https://github.com/rayon-rs/rayon). Previously I had relied upon manual threading + MPSC to achieve better performance but my code was littered with Arcs and RwLocks. Ironically I noticed that my multithreading model was actually performing worse than the single threaded version at the time did; and if you tried running it on a CPU that did not have exactly 16 cores the rendering was completely off.  

Rayon didn't just magically improve my performance but I actually had to rewrite my code and rethink my strategy altogether. When I tried to stuff MPSC into a rayon iterator I got an error. I couldn't understand how to fix it but I understood something much more important: Rayon made MPSC obsolete. Instead of sending a rendered pixel down the drain only to open a receiver after the rendering was done was not necessary.  
Instead I could just convert a range `0..100` to a list of rendered pixels with `map()` and `collect()`.  

I've rewritten a core part of the underlying structure of data. Previously only spheres were possible because that's the only shape covered in the aforementioned
tutorial. So I added the mother of all shapes: A triangle. Apparently calculating ray-triangle intersections in 3d space isn't as straightforward as 
a ray-sphere intersection which sounds really counterintuitive. So I chose the easy route and used the [Möller–Trumbore intersection algorithm](https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm). Copying the example C++ code to Rust is really straightforward.  

Triangles are loaded from wavefront .obj files. Note that the corresponding .mtl file has to exist next to the .obj file and faces have to be triangulated
during export.


## Performance  

The following image was generated from the current `main.rs` source code. Thus it has a resolution of 1280x720 with 500 samples per pixel. Running it in release mode on my Ryzen 2700x yields the following:  

```
$ time cargo run --release
    Finished release [optimized] target(s) in 0.03s
     Running `target/release/raytracer`
cargo run --release  326.15s user 2.19s system 1344% cpu 24.416 total
```  

24.5 seconds isn't too bad I think.  

![Example image](https://raw.githubusercontent.com/BrandtM/raytracer/master/images/example.png)  

And a example of the Blender monkey:  

![Monkey example image](https://raw.githubusercontent.com/BrandtM/raytracer/master/images/monkey.png)