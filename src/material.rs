use nalgebra::Vector3;
use crate::ray::Ray;
use rand::Rng;

pub struct CommonMaterial {
    pub emissive: Option<Vector3<f32>>,
    pub divert_ray: DivertRayMethod,
}

pub enum DivertRayMethod {
    Spec,
    Diff,
    DiffSpec(f32),
}

pub enum SeedingRay {
    DiffSpec(bool),
    NoSeed,
}

fn spec(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
    let d = (ray.d - norm * 2.0 * ray.d.dot(&norm)).normalize();
    Ray {d, o: o.clone()}
}

fn diff(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
    // cosine weighted hemisphere importance sampling based on https://www.mathematik.uni-marburg.de/~thormae/lectures/graphics1/code/ImportanceSampling/importance_sampling_notes.pdf
    let xd = (ray.d - norm * (ray.d.dot(&norm))).normalize();
    let yd = (norm.cross(&xd)).normalize();

    let mut rng = rand::thread_rng();
    let u: f32 = rng.gen();
    let v: f32 = rng.gen();

    let r = u.sqrt();
    let thet = 2.0 * std::f32::consts::PI * v;

    let x = r * thet.cos();
    let y = r * thet.sin();
    let d = (xd * x + yd * y + norm * (1.0 - u).max(0.0).sqrt()).normalize();

    Ray {d, o: o.clone()}
}

impl CommonMaterial {
    pub fn generate_seed(&self) -> SeedingRay {
        use DivertRayMethod::*;
        match self.divert_ray {
            Diff | Spec => {
                SeedingRay::NoSeed
            },
            DiffSpec(diffp) => {
                let mut rng = rand::thread_rng();
                let u: f32 = rng.gen();

                SeedingRay::DiffSpec(u < diffp)
            }
        }
    }
    pub fn should_dls(&self, seeding: &SeedingRay) -> bool {
        use DivertRayMethod::*;
        matches!((&self.divert_ray, seeding), (Diff, _) | (DiffSpec(_), SeedingRay::DiffSpec(true)))
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
            DiffSpec(_) => {
                if let SeedingRay::DiffSpec(should_diff) = seeding {
                    if *should_diff {
                        (diff(ray, norm, o), 1.0)
                    } else {
                        (spec(ray, norm, o), 1.0)
                    }
                } else {
                    panic!("seed should be set to DiffSpec!")
                }
            }
        }
    }
}