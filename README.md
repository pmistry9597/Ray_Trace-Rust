# Ray Tracing in Rust!

Ray traced rendering for realistic-looking images, all written in the Rust language!

**Comes with mesh rendering!**

<p float="middle">
    <img src="./info/images/stack.png" width="400" />
    <img src="./info/images/wada.png" width="400" />
    <img src="./info/images/james_webb.png" width="400" />
    <img src="./info/images/a380.png" width="400" />
</p>

*Licenses and sources for these rendered models can be found in the info/attribs folder.*

## 🚀🚀🚀 Features 🚀🚀🚀

- 🎇 Monte Carlo unidirectional path tracing
- Mesh loading via `.gltf` format! 💥💥💥
    - *some features in the `.gltf` format aren't properly implemented, however the models you see in the images worked fine for me*
- :fire: Accelerated ray tracing via k-d trees - *over 60x speedup for rendering complicated meshes!*
- Parallelized ray generation - via Rayon crate
- Dynamic scene building via yaml files
- Real-time display of rendering - stop whenever you're satisfied!
- Optional thin lens model for projection

<p float="middle">
    <img src="./info/images/discovery_space.png" width="400" />
    <img src="./info/images/biplane.png" width="400" />
    <img src="./info/images/spheres.png" width="400" />
    <img src="./info/images/voyager.png" width="400" />
    <img src="./info/images/wada_w_front.png" width="400" />
</p>

## How to Use

1. *Optionally,* find any `.gltf` models, note location of the `.gltf` file in the folder.
2. Create/modify a yaml file to describe a scheme to render (check out schemes folder for examples)
    - Basic settings
        ```yaml
        render_info:
            width: 1200
            height: 600
            samps_per_pix: 100000
            kd_tree_depth: 15 # play with this to minimize render time, more primitives means greater depth needed
            rad_info:  
                debug_single_ray: false
                dir_light_samp: false
                russ_roull_info:
                    assured_depth: 5
                    max_thres: 0.5
        ```
    - Camera
        ```yaml
        cam:
            o: [0, -15, -30] # position of camera
            view_eulers: [-0.6, 0.1, 0] # adjust where camera will look, rotates below settings
            d: [0, 0, 6] # render screen relative to position
            up: [0, 1, 0] # how will image be oriented? - should be unit vector !!!
            # following are in-scene screen dimensions - separate from width and height above!
            screen_width: 10.0
            screen_height: 5.0
        ```
    