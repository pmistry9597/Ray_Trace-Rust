use crate::ray::{Ray, HitResult};
use crate::elements::Renderable;

pub fn closest_ray_hit(ray: &Ray, elems: &Vec<Renderable>) -> (Vec<Option<HitResult>>, Option<usize>) {
    let hit_results: Vec<_> = elems.iter().map(|elem| elem.intersect(&ray)).collect();
    
    let i_hro = (&hit_results)
        .iter()
        .enumerate()
        .filter_map(|(i, hro)| {
            match hro {
                Some(hr) => {
                    if hr.l < (crate::EPS * 20.0).into() { // prevent immediate collision
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