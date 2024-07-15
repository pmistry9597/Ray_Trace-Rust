use nalgebra::Vector3;
use std::iter::zip;
use crate::render_target::RenderTarget;
use crate::ray::RayCompute;
use super::Scene;

use super::radiance::radiance;

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
            let (rgb, _) = radiance(&ray, &scene.objs, 0);

            pix.copy_from_slice(&rgb_f_to_u8(&rgb));
        });
    let elapsed = start.elapsed();
    println!("elapsed {:?}", elapsed);
}

fn rgb_f_to_u8(f: &Vector3<f32>) -> [u8; 4] {
    let mut out: [u8; 4] = [0; 4];
    // 255.0 * (1.0 - 1.0 / (f * 10.0 + 1.0))
    zip(out.iter_mut(), f.iter()).for_each(|(e, f)| *e = (f.clamp(0.0, 1.0) * 255.0 + 0.5).trunc() as u8); // assume 0.0 -> 1.0 range
    out
}