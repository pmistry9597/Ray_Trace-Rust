use image::{Pixel, Rgb32FImage};
use nalgebra::Vector3;

pub struct UVRgb32FImage (Rgb32FImage);

impl UVRgb32FImage {
    pub fn get_pixel(&self, u: f32, v: f32) -> Vector3<f32> {
        let face = &self.0;
        let width = face.width() as f32;
        let height = face.height() as f32;

        let rgb: Vec<f32> = face
            .get_pixel(
                (u * width).max(0.0).min(width-1.0).trunc() as u32,
                (v * height).max(0.0).min(height-1.0).trunc() as u32
                )
            .channels()
            .to_vec();
        let rgb: [f32; 3] = rgb.try_into().unwrap();
        rgb.into()
    }
}

impl From<Rgb32FImage> for UVRgb32FImage {
    fn from(im: Rgb32FImage) -> Self { UVRgb32FImage(im) }
}