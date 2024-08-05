use nalgebra::Vector3;
use crate::ray::Ray;
use rand::Rng;
use super::interaction::{diff, spec};

pub struct DynDiffSpec {}

impl DynDiffSpec {
    pub fn should_diff(diffp: f32) -> bool {
        let u: f32 = crate::RNG.with_borrow_mut(|r| r.gen());

        u < diffp
    }
    pub fn gen_new_ray(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>, should_diff: bool) -> (Ray, f32) {
        if should_diff {
            (diff(ray, norm, o), 1.0)
        } else {
            (spec(ray, norm, o), 1.0)
        }
    }
}