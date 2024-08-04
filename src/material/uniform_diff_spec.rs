use nalgebra::Vector3;
use crate::ray::Ray;
use rand::Rng;
use super::interaction::{diff, spec, refract};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UniformDiffuseSpec {
    pub emissive: Option<Vector3<f32>>,
    pub divert_ray: DivertRayMethod,
}

#[derive(Deserialize, Debug)]
pub enum DivertRayMethod {
    Spec,
    Diff,
    DiffSpec {diffp: f32},
    Dielectric {n_out: f32, n_in: f32},
}

pub enum SeedingRay {
    DiffSpec(bool),
    NoSeed,
}

impl UniformDiffuseSpec {
    pub fn generate_seed(&self) -> SeedingRay {
        use DivertRayMethod::*;
        match self.divert_ray {
            Diff | Spec | Dielectric {..} => {
                SeedingRay::NoSeed
            },
            DiffSpec {diffp} => {
                let mut rng = rand::thread_rng();
                let u: f32 = rng.gen();

                SeedingRay::DiffSpec(u < diffp)
            }
        }
    }
    pub fn should_dls(&self, seeding: &SeedingRay) -> bool {
        use DivertRayMethod::*;
        matches!((&self.divert_ray, seeding), (Diff, _) | (DiffSpec{..}, SeedingRay::DiffSpec(true)))
    }
    pub fn gen_new_ray(&self, ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>, seeding: &SeedingRay) -> (Ray, f32) {
        use DivertRayMethod::*;
        match self.divert_ray {
            Spec => {
                (spec(ray, norm, o), 1.0)
            },
            Diff => {
                (diff(ray, norm, o), 1.0)
            },
            DiffSpec {..} => {
                if let SeedingRay::DiffSpec(should_diff) = seeding {
                    if *should_diff {
                        (diff(ray, norm, o), 1.0)
                    } else {
                        (spec(ray, norm, o), 1.0)
                    }
                } else {
                    panic!("seed should be set to DiffSpec!")
                }
            },
            Dielectric {n_out, n_in} => {
                refract(ray, norm, o, &n_out, &n_in)
            },
        }
    }
}