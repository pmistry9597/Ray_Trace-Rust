use nalgebra::Vector3;
use crate::ray::Ray;

pub struct CommonMaterial {
    pub emissive: Option<Vector3<f32>>,
    pub spec_or_diff: SpecDiff,
}

pub enum SpecDiff {
    Spec,
    Diff,
    Both,
}

fn spec(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
    let d = ray.d - norm * 2.0 * ray.d.dot(&norm);
    Ray {d, o: o.clone()}
}

fn diff(ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
    // cosine weighted hemisphere importance sampling based on https://www.mathematik.uni-marburg.de/~thormae/lectures/graphics1/code/ImportanceSampling/importance_sampling_notes.pdf
    let xd = ray.d - norm * (ray.d.dot(&norm));
    let yd = xd.cross(norm);

    use rand::Rng;
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
    pub fn gen_new_ray(&self, ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>) -> Ray {
        use SpecDiff::*;
        match self.spec_or_diff {
            Spec => {
                spec(ray, norm, o)
            },
            Diff => {
                diff(ray, norm, o)
            },
            Both => {
                // CHANGE THIS
                unimplemented!();
            }
        }
    }
}