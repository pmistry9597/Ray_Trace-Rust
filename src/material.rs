use nalgebra::Vector3;
use crate::ray::Ray;
use rand::Rng;


use serde::Deserialize;
#[derive(Deserialize, Debug)]
pub struct CommonMaterial {
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

fn refract(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>, n_out: &f32, n_in: &f32) -> (Ray, f32) {
    // adapt from scratchapixel and smallpt
    // this helped a bit: https://blog.demofox.org/2020/06/14/casual-shadertoy-path-tracing-3-fresnel-rough-refraction-absorption-orbit-camera/
    let c_ = norm.dot(&ray.d);
    let into: bool = c_ < 0.0;
    let (n1, n2, c1, norm_refr) = if into {
        (*n_out, *n_in, -c_, norm.clone())
    } else {
        (*n_in, *n_out, c_, -norm)
    };
    let n_over = n1 / n2;
    let c22 = 1.0 - n_over * n_over * (1.0 - c1 * c1);

    let total_internal: bool = c22 < 0.0;
    let refl = spec(ray, &norm_refr, o);
    if total_internal {
        (refl, 1.0)
    } else {
        let trns = n_over * ray.d + norm_refr * (n_over * c1 - c22.sqrt()); // derived from snells law
        let r0 = ((n1 - n2) / (n1 + n2)).powf(2.0);
        let c = 1.0 - if into { c1 } else { trns.dot(norm) };
        let re = r0 + (1.0 + r0) * c.powf(5.0); // schlick approximation for reflection coef in fresnel equation
        
        let mut rng = rand::thread_rng();
        let u: f32 = rng.gen();

        if u < re {
            (refl, 1.0 / re)
        } else {
            (Ray {d: trns.normalize(), o: o.clone()}, 1.0 / (1.0 - re))
        }
    }
}

impl CommonMaterial {
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