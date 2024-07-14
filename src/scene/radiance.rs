use crate::ray::Ray;
use nalgebra::{Vector3, vector};
use super::Sphere;
use crate::ray::{Hitable, HasHitInfo};
use std::iter::zip;

type Obj = Sphere;

pub fn radiance(ray: &Ray, objs: &Vec<Obj>) -> Vector3<f32> { // color from a ray in a collection of hittable objects
    let hit_results: Vec<_> = objs.iter().map(|obj| obj.intersect(&ray)).collect();
    
    let obj_w_hit = zip(objs, &hit_results)
        .filter_map(|(o, hro)| {
            match hro {
                Some(hr) => Some((o, hr)),
                None => None,
            }
        })
        .min_by_key(|(_, hr)| hr.l.clone()); // closest hit result found here
    
    // use std::collections::HashSet;
    // let check_set = HashSet::from([(0,0), (1000,0)]);
    // if let Some((_obj, hit_result)) = obj_w_hit {
    //     if check_set.contains(&(x,y)) {
    //         let fuck: f32 = hit_result.l.clone().into();
    //         println!("pixel: {:?}, ray len: {}", (x,y), fuck);
    //     }
    // }

    if let Some((obj, hit_result)) = obj_w_hit { 
        obj.hit_info(hit_result).rgb 
    } else { 
        vector![0.0, 0.0, 0.0] 
    }
}