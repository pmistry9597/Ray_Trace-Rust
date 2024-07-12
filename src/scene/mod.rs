use nalgebra::{Vector3, vector};
use crate::render_target::RenderTarget;
use crate::ray::{RayCompute, Hitable, HasHitInfo};
use basic_shape::Sphere;
use std::iter::zip;

mod basic_shape;

pub fn render_to_target(render_target: &RenderTarget, scene: &Scene) {
    use rayon::prelude::*;

    let ray_compute = RayCompute::new(&render_target, &scene.cam);

    use std::time::Instant;
    let start = Instant::now();
    render_target.buff_mux.lock()
        .par_chunks_mut(4) // pixels have rgba values, so chunk by 4
        .enumerate()
        .map(|(i, pix)| (render_target.chunk_to_pix(i.try_into().unwrap()), pix))
        .for_each(|((x, y), pix)| {
            let ray = ray_compute.pix_cam_to_ray((x,y), &scene.cam);

            let hit_results: Vec<_> = scene.objs.iter().map(|sph| sph.intersect(&ray)).collect();
            
            let obj_w_hit = zip(&scene.objs, &hit_results)
                .filter_map(|(o, hro)| {
                    match hro {
                        Some(hr) => Some((o, hr)),
                        None => None,
                    }
                })
                .min_by_key(|(_, hr)| hr.l.clone()); // closest hit result found here
            let rgb = if let Some((obj, hit_result)) = obj_w_hit { obj.hit_info(hit_result).rgb } else { vector![0.0, 0.0, 0.0] };

            // let dat: [u8; 4] = [200, 0, 100, 0];
            pix.copy_from_slice(&rgb_f_to_u8(&rgb));
        });
    let elapsed = start.elapsed();
    println!("elapsed {:?}", elapsed);
}

fn rgb_f_to_u8(f: &Vector3<f32>) -> [u8; 4] {
    let mut out: [u8; 4] = [0; 4];
    zip(out.iter_mut(), f.iter()).for_each(|(e, f)| *e = (f * 255.0).trunc() as u8); // assume 0.0 -> 1.0 range
    out
}

pub fn give_crap() -> Scene {
    // let pee: Vector3<f32> = vector![1.0,1.0,1.0];
    use basic_shape::Coloring::*;
    use std::sync::Arc;
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
                // Sphere{c: vector![-10.0, -5.0, -25.0], r: 1.0},
                // Sphere{c: vector![10.0, -5.0, -25.0], r: 1.0},
                // Sphere{c: vector![10.0, 5.0, -25.0], r: 1.0},
                // Sphere{c: vector![-10.0, 5.0, -25.0], r: 1.0},

                Sphere{c: vector![-10.0, -5.0, -30.0], r: 1.0, 
                    coloring: UsePos(Arc::new(
                        |pos, sph| vector![0.8, 0.8*(pos[0] + sph.r - sph.c[0]).abs()/(2.0*sph.r), 0.5]))
                },
                Sphere{c: vector![10.0, -5.0, -30.0], r: 1.0, 
                    coloring: UsePos(Arc::new(
                        |pos, sph| vector![(pos[2] + sph.r - sph.c[2]).abs()/(2.0*sph.r), 0.0, 0.1]))
                },
                Sphere{c: vector![10.0, 5.0, -30.0], r: 1.0, 
                    coloring: UsePos(Arc::new(
                        |pos, sph| vector![(pos[2] + sph.r - sph.c[2]).abs()/(2.0*sph.r), 0.6, 0.0]))
                },
                Sphere{c: vector![-10.0, 5.0, -30.0], r: 1.0, coloring: Solid(vector![0.6, 0.0, 1.0])},

                // Sphere{c: vector![10.0, -5.0, -20.0], r: 1.0, coloring: Solid(vector![1.0, 0.0, 0.6])},
                // Sphere{c: vector![10.0, 5.0, -20.0], r: 1.0, coloring: Solid(vector![1.0, 0.2, 0.6])},
                Sphere{c: vector![-10.0, 5.0, -20.0], r: 1.0, 
                    coloring: UsePos(Arc::new(
                        |pos, sph| vector![(pos[1] - sph.c[1]).abs()/sph.r, 0.9, 0.1]))
                },

                Sphere{c: vector![2.0, 0.5, -10.0], r: 4.0, 
                    coloring: UsePos(Arc::new(
                        |pos, sph| vector![(pos[2] - sph.c[2]).abs()/sph.r, 0.2, 0.8]))
                },
                Sphere{c: vector![2.0, 0.5, -6.0], r: 1.0, 
                    coloring: UsePos(Arc::new(
                        |pos, sph| vector![0.3, (pos[0] + sph.r - sph.c[0]).abs()/(2.0*sph.r), 0.8]))
                },
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