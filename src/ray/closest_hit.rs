use crate::ray::{Ray, HitResult};
use crate::elements::Renderable;

pub type ClosestRayHit = (Vec<(usize, Option<HitResult>)>, Option<usize>);

pub fn closest_ray_hit<'r, I: Iterator<Item = (usize, Renderable<'r>)>>(ray: &Ray, elems: I) -> ClosestRayHit {
    let hit_results: Vec<_> = elems.map(|(i, e)| (i, e.intersect(&ray))).collect();
    
    let hro = {
        let i_hro = (&hit_results)
            .iter()
            // .enumerate()
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
            
        i_hro.map(|(i, _)| *i)
    };
    (hit_results, hro)
}