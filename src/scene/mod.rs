use nalgebra::{Vector3, vector};
use crate::render_target::RenderTarget;
use crate::ray::{RayCompute, Hitable};
use basic_shape::Sphere;

mod basic_shape;

pub fn render_to_target(render_target: &RenderTarget, scene: &Scene) {
    use rayon::prelude::*;

    let ray_compute = RayCompute::new(&render_target, &scene.cam);

    render_target.buff_mux.lock()
        .par_chunks_mut(4) // pixels have rgba values, so chunk by 4
        .enumerate()
        .map(|(i, pix)| (render_target.chunk_to_pix(i.try_into().unwrap()), pix))
        .for_each(|((x, y), pix)| {
            let ray = ray_compute.pix_cam_to_ray((x,y), &scene.cam);

            let dat: [u8; 4] = if scene.objs.iter().any(|sph| sph.intersect(&ray).is_some()) {
                [100, 50, 255, 0]
            } else {
                [0, 0, 0, 0]
            };

            // let dat: [u8; 4] = [200, 0, 100, 0];
            pix.copy_from_slice(&dat);
        });
}

pub fn give_crap() -> Scene {
    // let pee: Vector3<f32> = vector![1.0,1.0,1.0];
    let cam = Cam {
        d: vector![0.0, 0.0, -5.0],
        o: vector![0.0, 0.0, 0.0],
        up: vector![0.0, 1.0, 0.0].normalize(),
        screen_width: 10.0,
        screen_height: 5.0,
    };
    Scene {
        cam,
        objs: vec![
                Sphere{c: vector![-10.0, -5.0, -20.0], r: 1.0},
                Sphere{c: vector![10.0, -5.0, -20.0], r: 1.0},
                Sphere{c: vector![10.0, 5.0, -20.0], r: 1.0},
                Sphere{c: vector![-10.0, 5.0, -20.0], r: 1.0},

                // Sphere{c: vector![-10.0, -5.0, -25.0], r: 1.0},
                // Sphere{c: vector![10.0, -5.0, -25.0], r: 1.0},
                // Sphere{c: vector![10.0, 5.0, -25.0], r: 1.0},
                // Sphere{c: vector![-10.0, 5.0, -25.0], r: 1.0},

                Sphere{c: vector![-10.0, -5.0, -30.0], r: 1.0},
                Sphere{c: vector![10.0, -5.0, -30.0], r: 1.0},
                Sphere{c: vector![10.0, 5.0, -30.0], r: 1.0},
                Sphere{c: vector![-10.0, 5.0, -30.0], r: 1.0},
            ],
    }
}

pub struct Cam {
    pub d: Vector3<f32>, // o -> center of screen, has distance
    pub o: Vector3<f32>,
    pub up: Vector3<f32>, // should be unit vector
    // in-scene dimensions, not view pixels
    pub screen_width: f32, 
    pub screen_height: f32,
}

pub struct Scene {
    pub cam: Cam,
    objs: Vec<Sphere>,
}