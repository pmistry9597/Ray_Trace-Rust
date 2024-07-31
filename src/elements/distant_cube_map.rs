use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo, InteractsWithRay, DLSEmitter};
use crate::elements::IsCompleteElement;
use image::{ImageBuffer, Pixel};

type FaceImage = ImageBuffer<image::Rgb<f32>, Vec<f32>>;

// for environment mapping, all rays can hit this
pub struct DistantCubeMap {
    pub neg_z: FaceImage,
    pub pos_z: FaceImage,
    pub neg_x: FaceImage,
    pub pos_x: FaceImage,
    pub neg_y: FaceImage,
    pub pos_y: FaceImage,
}

impl IsCompleteElement for DistantCubeMap {}

impl InteractsWithRay for DistantCubeMap {
    fn shoot_new_ray(&self, _ray: &Ray, _hit_info: &HitInfo) -> Option<(Ray, f32)> { None } // cant shoot new ray silly
    fn give_dls_emitter(&self) -> Option<Box<dyn DLSEmitter + '_>> { None } // maybe ill do this? for a skybox it seems almost unnecessary since all rays can hit
}

impl HasHitInfo for DistantCubeMap {
    fn hit_info(&self, _info: &HitResult, ray: &Ray) -> HitInfo {
        use nalgebra::vector;
        
        let comps: &[f32] = (&ray.d).into();
        let (max_idx, max_c) = comps.iter().enumerate()
            .reduce(|(prev_i, prev_c), (i, c)| if c.abs() > prev_c.abs() {(i, c)} else {(prev_i, prev_c)})
            .unwrap();
        
        use std::cmp::Ordering;
        let (u, v, fact, face) = match (max_idx, max_c.partial_cmp(&0.0).expect(&format!("wtf {}", max_c))) {
            (0, Ordering::Less) => {
                let d = ray.d.normalize();
                let (u, v) = (d[2], d[1]);
                let fact = d[0];

                (u, v, fact, &self.neg_x)
            },
            (0, Ordering::Greater) => {
                let d = ray.d.normalize();
                let (u, v) = (d[2], d[1]);
                let fact = d[0];

                (u, -v, fact, &self.pos_x)
            },
            (1, Ordering::Less) => {
                let d = ray.d.normalize();
                let (u, v) = (d[0], d[2]);
                let fact = d[1];

                (-u, -v, fact, &self.neg_y)
            },
            (1, Ordering::Greater) => {
                let d = ray.d.normalize();
                let (u, v) = (d[0], d[2]);
                let fact = d[1];

                (u, -v, fact, &self.pos_y)
            },
            (2, Ordering::Less) => {
                let d = ray.d.normalize();
                let (u, v) = (d[0], d[1]);
                let fact = d[2];

                (-u, v, fact, &self.neg_z)
            },
            (2, Ordering::Greater) => {
                let d = ray.d.normalize();
                let (u, v) = (d[0], d[1]);
                let fact = d[2];

                (-u, -v, fact, &self.pos_z)
            },
            _ => { panic!("this should be impossible!!") },
        };

        HitInfo {
            rgb: vector![0.0,0.0,0.0],
            emissive: sample_face(u, v, fact, face), //: vector![0.7,0.7,1.0] * atten + red_comp,
            pos: ray.d * f32::INFINITY,
            norm: -ray.d,
            dls: false,
            bounce_info: None,
        }
    }
}

fn sample_face(u: f32, v: f32, fact: f32, face: &FaceImage) -> Vector3<f32> {
    let width = face.width() as f32;
    let height = face.height() as f32;

    let (u, v) = (0.5 * u / fact + 0.5, 0.5 * v / fact + 0.5);
    let rgb: Vec<f32> = face.get_pixel((u * width).min(width-1.0).trunc() as u32, (v * height).min(height-1.0).trunc() as u32).channels().to_vec();
    let rgb: [f32; 3] = rgb.try_into().unwrap();
    rgb.into()
}

impl Hitable for DistantCubeMap {
    fn intersect(&self, _ray: &Ray) -> Option<HitResult> { // always hits since distant and covers all
        Some(HitResult{l: f32::INFINITY.into(), intermed: None})
    }
}