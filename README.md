# Ray Tracing in Rust!

Ray traced rendering for realistic-looking images, all written in the Rust language!

**Comes with mesh rendering!**

<p float="middle">
    <img src="./info/images/stack.png" width="400" />
    <img src="./info/images/wada.png" width="400" />
    <img src="./info/images/james_webb.png" width="400" />
    <img src="./info/images/a380.png" width="400" />
</p>

*Licenses and links for these rendered models can be found in the info/attribs folder.*

## Features 🚀🚀🚀

- Monte Carlo unidirectional path tracing
- Mesh loading via .gltf files! 💥💥💥
    - *some features in the models aren't properly implemented, however the models you see in the images worked fine for me*
- 🔥 Accelerated ray tracing via k-d trees - *> 60x speedup for rendering complicated meshes!*
- Parallelized ray generation - via Rayon crate
- Dynamic scene building via yaml files
- Real-time display of rendering - stop whenever you're satisfied!
- Optional thin lens model for projection

<p float="middle">
    <img src="./info/images/discovery_space.png" width="400" />
    <img src="./info/images/biplane.png" width="400" />
    <img src="./info/images/spheres.png.png" width="400" />
    <img src="./info/images/voyager.png" width="400" />
    <img src="./info/images/wada_w_front.png" width="400" />
</p>