use crate::ray::Ray;
use nalgebra::{Vector3, vector};
use super::basic_shape::{Sphere};
use crate::ray::{Hitable, HasHitInfo, InteractsWithRay, HitResult, HitInfo};
// use std::iter::zip;
use rand::Rng;

type Element = Sphere;

pub fn radiance(ray: &Ray, elems: &Vec<Element>, depth: i32, dir_light_samp: bool) -> (Vector3<f32>, Option<usize>) { // color from a ray in a collection of hittable objects, and index of object that was hit
    let (hit_results, idxo) = closest_ray_hit(ray, elems);
    
    // use std::collections::HashSet;
    // let check_set = HashSet::from([(0,0), (1000,0)]);
    // if let Some((_obj, hit_result)) = obj_w_hit {
    //     if check_set.contains(&(x,y)) {
    //         let fuck: f32 = hit_result.l.clone().into();
    //         println!("pixel: {:?}, ray len: {}", (x,y), fuck);
    //     }
    // }
    
    if let Some(elem_idx) = idxo { 
        let elem = &elems[elem_idx];
        let hit_result = &hit_results[elem_idx].as_ref().unwrap();
        let (hit_info, roull_pass) = russian_roulette_filter(depth, elem.hit_info(hit_result, ray));

        if roull_pass {
            match hit_info.bounce_info {
                Some(_) => {
                    let (new_ray, p) = elem.shoot_new_ray(ray, &hit_info);
                    let (incoming_rgb, incoming_idx) = radiance(&new_ray, elems, depth + 1, dir_light_samp);
    
                    let mul = if dir_light_samp && hit_info.dls {
                        let omit_idxs = if incoming_idx.is_some() {
                            vec![elem_idx, incoming_idx.unwrap()]
                        } else {
                            vec![elem_idx]
                        };
                        let light_contrib = establish_dls_contrib(&omit_idxs, elems, &hit_info, ray);
                        incoming_rgb / p + light_contrib
                    } else {
                        incoming_rgb / p
                    };
                    // let mul = incoming_rgb / p;
    
                    (hit_info.emissive + hit_info.rgb.component_mul(&mul), Some(elem_idx))
                },
                None => {
                    (hit_info.emissive, Some(elem_idx))
                }
            }
        } else {
            (hit_info.emissive, Some(elem_idx))
        }
    } else { 
        (vector![0.0, 0.0, 0.0], None)
    }
}

fn russian_roulette_filter<T>(depth: i32, mut hit_info: HitInfo<T>) -> (HitInfo<T>, bool) {
    if depth > 5 {
        let mut rng = rand::thread_rng();
        let russ_roull: f32 = rng.gen();
        // let thres = hit_info.rgb.iter().reduce(|prev, e| if e > prev {e} else {prev}).expect("ain't there a max color??");
        let thres: f32 = 0.8;
        // println!("russian rollete {} {}", russ_roull, thres);
        if russ_roull < thres {
            hit_info.rgb = hit_info.rgb / thres; // monte carlo normalizing term for when russian roulette active
            (hit_info, true)
        } else {
            (hit_info, false)
        }
    } else {
        (hit_info, true)
    }    
}

// direct light sampling based on https://iquilezles.org/articles/simplepathtracing/
fn establish_dls_contrib<T>(omit_idxs: &[usize], elems: &Vec<Element>, hit_info: &HitInfo<T>, ray: &Ray) -> Vector3<f32> {
    const NORMZE: f32 = 1.0 / (30.0 * std::f32::consts::PI);

    // only use lights and dont use self
    let lights = elems.iter().enumerate()
        .filter(|(i,o)| o.emits() && !matches!(omit_idxs.iter().position(|e| *e == *i), Some(_)));

    lights.fold(vector![0.0,0.0,0.0], |a, (i,l)| {
        let d = (l.c - hit_info.pos).normalize(); // NOTE: change l.c to be sample from light, no assumption that it is a sphere
        let light_dot = d.dot(&hit_info.norm);

        if light_dot > 0.0 {
            let dls_ray = Ray{ d, o: hit_info.pos }; 
            let (hrs, idxo) = closest_ray_hit(&dls_ray, elems);

            if let Some(idx) = idxo {
                if i == idx { // make sure its the same light source!!
                    let hit_info = elems[idx].hit_info(&hrs[idx].as_ref().unwrap(), ray);
                    // let cos_a_max = 2.0 * std::f32::consts::PI * (1.0 - objs[idx].r.powf(2.0) / (hit_info.pos - objs[idx].c).dot(&(hit_info.pos - objs[idx].c)));
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

// results of ray closest object closest intersection
fn closest_ray_hit(ray: &Ray, elems: &Vec<Element>) -> (Vec<Option<HitResult<Vector3<f32>>>>, Option<usize>) {
    let hit_results: Vec<_> = elems.iter().map(|elem| elem.intersect(&ray)).collect();
    
    let i_hro = (&hit_results)
        .iter()
        .enumerate()
        .filter_map(|(i, hro)| {
            match hro {
                Some(hr) => {
                    if hr.l < (crate::EPS * 10.0).into() { // prevent immediate collision
                        None
                    } else {
                        Some((i, hr))
                    }
                },
                None => None,
            }
        })
        .min_by_key(|(_, hr)| hr.l.clone()); // closest hit result found here
    let hro = match i_hro {
        Some((i, _)) => Some(i),
        None => None,
    };
    (hit_results, hro)
}