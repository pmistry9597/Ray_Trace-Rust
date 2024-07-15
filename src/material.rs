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

fn spec(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
    let d = ray.d - norm * 2.0 * ray.d.dot(&norm);
    Ray {d, o: o.clone()}
}

fn diff(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
    // cosine weighted hemisphere importance sampling based on https://www.mathematik.uni-marburg.de/~thormae/lectures/graphics1/code/ImportanceSampling/importance_sampling_notes.pdf
    let xd = ray.d - norm * (ray.d.dot(&norm));
    let yd = xd.cross(norm);

    let mut rng = rand::thread_rng();
    let u: f32 = rng.gen();
    let v: f32 = rng.gen();

    let phi = std::f32::consts::PI * 2.0 * u;
    let thet = v.sqrt().asin();

    let cphi = phi.cos();
    let sthet = thet.sin();
    let sphi = phi.sin();
    let d = xd * (cphi * sthet) + yd * (sphi * sthet) + norm * thet.cos();

    Ray {d, o: o.clone()}
}

impl CommonMaterial {
    pub fn gen_new_ray(&self, ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> (Ray, f32) {
        use DivertRayMethod::*;
        match self.divert_ray {
            Spec => {
                (spec(ray, norm, o), 1.0)
            },
            Diff => {
                (diff(ray, norm, o), 1.0)
            },
            DiffSpec(diffp) => {
                let mut rng = rand::thread_rng();
                let u: f32 = rng.gen();

                if u < diffp {
                    (diff(ray, norm, o), diffp)
                } else {
                    (spec(ray, norm, o), 1.0 - diffp)
                }
            }
        }
    }
}