use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo, InteractsWithRay, DLSEmitter};
use crate::elements::IsCompleteElement;
use image::{ImageBuffer, Pixel};

type FaceImage = ImageBuffer<image::Rgb<f32>, Vec<f32>>;
pub type FaceImagewUVScale = (FaceImage, f32, f32);

// for environment mapping, all rays can hit this
pub struct DistantCubeMap {
    pub neg_z: FaceImagewUVScale,
    pub pos_z: FaceImagewUVScale,
    pub neg_x: FaceImagewUVScale,
    pub pos_x: FaceImagewUVScale,
    pub neg_y: FaceImagewUVScale,
    pub pos_y: FaceImagewUVScale,
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
        
        let d = ray.d.normalize();
        use std::cmp::Ordering;
        let (u, v, fact, face) = 
            match (max_idx, max_c.partial_cmp(&0.0).expect(&format!("wtf {}", max_c))) {
                (0, Ordering::Less) => (d.z, d.y, d.x, &self.neg_x),

                (0, Ordering::Greater) => (d.z, d.y, d.x, &self.pos_x),

                (1, Ordering::Less) => (d.x, d.z, d.y, &self.neg_y),

                (1, Ordering::Greater) => (d.x, d.z, d.y, &self.pos_y),

                (2, Ordering::Less) => (d.x, d.y, d.z, &self.neg_z),

                (2, Ordering::Greater) => (d.x, d.y, d.z, &self.pos_z),

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

fn sample_face(u: f32, v: f32, fact: f32, facewscale: &FaceImagewUVScale) -> Vector3<f32> {
    let (_, us, vs) = *facewscale;
    let face = &facewscale.0;
    let width = face.width() as f32;
    let height = face.height() as f32;

    let (u, v) = (0.5 * u * us / fact + 0.5, 0.5 * v * vs / fact + 0.5);
    let rgb: Vec<f32> = face.get_pixel((u * width).min(width-1.0).trunc() as u32, (v * height).min(height-1.0).trunc() as u32).channels().to_vec();
    let rgb: [f32; 3] = rgb.try_into().unwrap();
    rgb.into()
}

impl Hitable for DistantCubeMap {
    fn intersect(&self, _ray: &Ray) -> Option<HitResult> { // always hits since distant and covers all
        Some(HitResult{l: f32::INFINITY.into(), intermed: None})
    }
}