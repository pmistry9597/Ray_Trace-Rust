use nalgebra::{Vector3, vector};
use crate::ray::{Ray, closest_ray_hit, HitInfo};
use crate::elements::Renderable;
use rand::Rng;

use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct RadianceInfo {
    pub debug_single_ray: bool,
    pub dir_light_samp: bool,
    pub russ_roull_info: RussianRoullInfo,
}
#[derive(Deserialize, Debug)]
pub struct RussianRoullInfo {
    pub assured_depth: i32,
    pub max_thres: f32,
}

pub fn radiance(ray: &Ray, elems: &Vec<Renderable>, depth: i32, rad_info: &RadianceInfo) -> (Vector3<f32>, Option<usize>) { // color from a ray in a collection of hittable objects, and index of object that was hit
    let (hit_results, idxo) = closest_ray_hit(ray, elems.into_iter().enumerate().map(|(i, r)| (i, *r)));
    
    if let Some(elem_idx) = idxo { 
        let elem = &elems[elem_idx];
        let hit_result = &hit_results[elem_idx].1.as_ref().unwrap();
        let hit_info = elem.hit_info(hit_result, ray);

        if rad_info.debug_single_ray {
            (hit_info.emissive, Some(elem_idx))
        } else {
            let (roull_pass, atten) = russian_roulette_filter(depth, &rad_info.russ_roull_info);
            
            if roull_pass {
                match hit_info.continue_info {
                    Some(_) => {
                        let (rgb, new_ray) = elem.continue_ray(ray, &hit_info).expect("cant shoot a ray??");
                        let rgb = match atten {
                            Some(f) => rgb / *f,
                            None => rgb,
                        };
                        let (incoming_rgb, incoming_idx) = radiance(&new_ray, elems, depth + 1, rad_info);
        
                        let mul = if rad_info.dir_light_samp && hit_info.dls {
                            let omit_idxs = if incoming_idx.is_some() {
                                vec![elem_idx, incoming_idx.unwrap()]
                            } else {
                                vec![elem_idx]
                            };
                            let light_contrib = establish_dls_contrib(&omit_idxs, elems, &hit_info, ray);
                            incoming_rgb + light_contrib
                        } else {
                            incoming_rgb
                        };
                        // let mul = incoming_rgb / p;
        
                        (hit_info.emissive + rgb.component_mul(&mul), Some(elem_idx))
                    },
                    None => {
                        (hit_info.emissive, Some(elem_idx))
                    }
                }
            } else {
                (hit_info.emissive, Some(elem_idx))
            }
        }
    } else { 
        (vector![0.0, 0.0, 0.0], None)
    }
}

fn russian_roulette_filter(depth: i32, russ_roull_info: &RussianRoullInfo) -> (bool, Option<&f32>) { // second is normalizing term for rgb value should russian roullete be done
    if depth > russ_roull_info.assured_depth {
        let russ_roull: f32 = crate::RNG.with_borrow_mut(|r| r.gen());
        static THRES: f32 = 0.4;
        if russ_roull < THRES {
            (true, Some(&THRES))
        } else {
            (false, None)
        }
    } else {
        (true, None)
    }    
}

// direct light sampling based on https://iquilezles.org/articles/simplepathtracing/
fn establish_dls_contrib(omit_idxs: &[usize], elems: &Vec<Renderable>, hit_info: &HitInfo, ray: &Ray) -> Vector3<f32> {
    const NORMZE: f32 = 1.0 / (30.0 * std::f32::consts::PI);

    // only use valid lights
    let emitters = elems.iter().enumerate()
        .map(|(i,o)| (i, o.give_dls_emitter()))
        .filter(|(i,e)| e.is_some() && !omit_idxs.contains(i))
        .map(|(i,e)| (i, e.unwrap()));

    emitters.fold(vector![0.0,0.0,0.0], |a, (i,emitter)| {
        let d = emitter.dls_ray(&hit_info.pos, &hit_info.norm);
        let light_dot = d.dot(&hit_info.norm);

        if light_dot > 0.0 {
            let dls_ray = Ray{ d, o: hit_info.pos }; 
            let (hrs, idxo) = closest_ray_hit(&dls_ray, elems.into_iter().enumerate().map(|(i, r)| (i, *r)));

            if let Some(idx) = idxo {
                if i == idx { // make sure its the same light source!!
                    let hit_info = elems[idx].hit_info(&hrs[idx].1.as_ref().unwrap(), ray);
                    a + light_dot * hit_info.emissive * NORMZE // brdf-esque normalizing, perhaps ask object for brdf value for a ray?
                } else {
                    a
                }
            } else {
                a
            }
        } else {
            a
        }
    })
}