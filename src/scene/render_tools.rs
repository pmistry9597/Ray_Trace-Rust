use std::iter::zip;
use crate::render_target::RenderTarget;
use crate::ray::RayCompute;
use super::Scene;

use super::radiance::radiance;

pub fn render_to_target<F : Fn() -> ()>(render_target: &RenderTarget, scene: &Scene, update_hook: F) {
    use rayon::prelude::*;

    let ray_compute = RayCompute::new(&render_target, &scene.cam);

    use std::time::Instant;
    let start = Instant::now();

    render_target.buff_mux.lock().fill(0);
    let mut sample_count: f32 = 0.0;
    let mut target: Vec<[f32; 3]> = [[0.0, 0.0, 0.0]].repeat((render_target.canv_width * render_target.canv_height).try_into().unwrap());

    let num_samples = 100000;
    for r_it in 0..num_samples {
        target.par_iter_mut()
            .enumerate()
            .map(|(i, pix)| (render_target.chunk_to_pix(i.try_into().unwrap()), pix))
            .for_each(|((x, y), pix)| {
                let ray = ray_compute.pix_cam_to_rand_ray((x,y), &scene.cam);
                let (rgb, _) = radiance(&ray, &scene.objs, 0);
                let rgb: Vec<f32> = rgb.iter().copied().collect();

                zip(pix.iter_mut(), &rgb).for_each(|(p, r)| {
                    *p = (r + (*p * sample_count)) / (sample_count + 1.0);
                });
            });

        sample_count += 1.0;

        render_target.buff_mux.lock()
            .par_chunks_mut(4) // pixels have rgba values, so chunk by 4
            .zip(&target)
            .for_each(|(pix, tar)| pix.copy_from_slice(&rgb_f_to_u8(tar)));

        update_hook();
        println!("render iteration {}: {:?}", r_it, start.elapsed());
    }

    let elapsed = start.elapsed();
    println!("elapsed {:?}", elapsed);
}

fn rgb_f_to_u8(f: &[f32]) -> [u8; 4] {
    let mut out: [u8; 4] = [0; 4];
    // 255.0 * (1.0 - 1.0 / (f * 10.0 + 1.0))
    zip(out.iter_mut(), f.iter()).for_each(|(e, f)| *e = (f.clamp(0.0, 1.0) * 255.0 + 0.5).trunc() as u8); // assume 0.0 -> 1.0 range
    out
}