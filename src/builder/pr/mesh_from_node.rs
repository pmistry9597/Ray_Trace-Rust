use nalgebra::Vector3;
use serde::Deserialize;
use crate::elements::mesh::{Mesh, PbrMetalRoughInfo, RgbInfo, NormInfo};
use image::{DynamicImage, ImageBuffer};
use nalgebra::Vector2;
use crate::material::UVRgb32FImage;

// --- -------- ------- - -- ----- - ----- FUCK --------------
// this file should be deleted/changed around soon!!
// --- --- --- --PEE ----- --- ----- ----

#[derive(Deserialize, Debug)]
pub struct MeshFromNode {
    path: String,
    node_index: usize,
    uniform_scale: f32,
}

impl MeshFromNode {
    pub fn to_mesh(&self) -> Mesh {
        let (document, buffers, images) = gltf::import(&self.path).unwrap();
        let node_oi = document.nodes().nth(self.node_index).unwrap();

        let mesh = node_oi.mesh().unwrap();
        // let primitives = mesh.primitives(); <--- soon split into multiple meshes based on different primitives

        let primitive = mesh.primitives().next().unwrap();
        let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()].0));

        let flat_indices: Vec<usize> = reader.read_indices().unwrap()
                .into_u32()
                .map(|v| v.try_into().unwrap())
                .collect();

        let poses: Vec<Vector3<f32>> = reader.read_positions().unwrap().map(|p| p.into()).collect();
        let poses: Vec<Vector3<f32>> = poses.iter().map(|v| v * self.uniform_scale ).collect();

        let material = primitive.material();
        let pbr_met_rough = material.pbr_metallic_roughness();
        
        let (textures, tex_coords) = texinfo_to_uvtex_and_coords(&pbr_met_rough.base_color_texture(), &reader, &images);
        let base_color_factor: [f32; 3] = pbr_met_rough.base_color_factor()[..3].try_into().unwrap();
        let rgb_info = RgbInfo {
            factor: base_color_factor.into(),
            coords: tex_coords,
        };
        let (normal_maps, norm_info) = match material.normal_texture() {
            Some(n_info) => {
                let (normal_maps, norm_coords) = get_uvtex_and_coords(&n_info.texture(), n_info.tex_coord(), &reader, &images);
                (normal_maps, Some(NormInfo { scale: n_info.scale(), coords: norm_coords.unwrap() }))
            },
            None => {
                (vec![], None)
            },
            };

        let tangents: Option<Vec<[f32; 3]>> = reader.read_tangents().map(|tans| tans.map(|t| t[..3].try_into().unwrap()).collect());

        let (metal_rough_maps, mr_coords) = texinfo_to_uvtex_and_coords(&pbr_met_rough.metallic_roughness_texture(), &reader, &images);
        let metal_rough = PbrMetalRoughInfo {
            metal: pbr_met_rough.metallic_factor(),
            rough: pbr_met_rough.roughness_factor(),
            coords: mr_coords,
        };

        Mesh {
            poses,
            norms: reader.read_normals().unwrap().map(|p| p.into()).collect(),
            indices: flat_indices.chunks(3).map(|c| c.try_into().unwrap()).collect(),
            rgb_info,
            norm_info,
            tangents: tangents.map(|t| t.iter().map(|ta| (*ta).into()).collect()), 
            metal_rough,
            
            textures,
            normal_maps,
            metal_rough_maps,
        }
    }
}

use gltf::texture::Info;
use gltf::mesh::Reader;
use gltf::image::Data;
use gltf::{Buffer, Texture};

fn texinfo_to_uvtex_and_coords<'a, 's, F>(tex_info: &Option<Info>, reader: &Reader<'a, 's, F>, images: &Vec<Data>) -> (Vec<UVRgb32FImage>, Option<Vec<Vector2<f32>>>) 
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    match tex_info {
        Some(info) => get_uvtex_and_coords(&info.texture(), info.tex_coord(), reader, images),
        None => (vec![], None),
    }
}

fn get_uvtex_and_coords<'a, 's, F>(texture: &Texture, tex_coord: u32, reader: &Reader<'a, 's, F>, images: &Vec<Data>) -> (Vec<UVRgb32FImage>, Option<Vec<Vector2<f32>>>)
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    let coords: Vec<Vector2<f32>> = reader.read_tex_coords(tex_coord).expect("no metal roughness map coordinates?").into_f32().map(|p| p.into()).collect();

    let image_data = &images[texture.index()];
    use gltf::image::Format::*;
    println!("format!!!! : {:?}", image_data.format);
    let dyn_image = match image_data.format {
        R8 => DynamicImage::ImageLuma8(
            ImageBuffer::from_raw(image_data.width, image_data.height, image_data.pixels.clone()).expect("doesn't fit??")
        ),
        R8G8B8 => DynamicImage::ImageRgb8(
            ImageBuffer::from_raw(image_data.width, image_data.height, image_data.pixels.clone()).expect("doesn't fit??")
        ),
        R8G8B8A8 => DynamicImage::ImageRgba8(
            ImageBuffer::from_raw(image_data.width, image_data.height, image_data.pixels.clone()).expect("doesn't fit??")
        ),
        R16G16B16 => DynamicImage::ImageRgb16(
            ImageBuffer::from_raw(image_data.width, image_data.height, 
                image_data.pixels.clone()
                .chunks(2)
                .map(|c| unsafe { std::mem::transmute::<[u8; 2], u16>(c.try_into().unwrap()) })
                .collect())
                .expect("doesn't fit??")
        ),
        R16G16B16A16 => DynamicImage::ImageRgba16(
            ImageBuffer::from_raw(image_data.width, image_data.height, 
                image_data.pixels.clone()
                .chunks(2)
                .map(|c| unsafe { std::mem::transmute::<[u8; 2], u16>(c.try_into().unwrap()) })
                .collect())
                .expect("doesn't fit??")
        ),
        R32G32B32FLOAT => DynamicImage::ImageRgb32F(
            ImageBuffer::from_raw(image_data.width, image_data.height, 
                image_data.pixels.clone()
                .chunks(4)
                .map(|c| unsafe { std::mem::transmute::<[u8; 4], f32>(c.try_into().unwrap()) })
                .collect())
                .expect("doesn't fit??")
        ),
        R32G32B32A32FLOAT => DynamicImage::ImageRgba32F(
            ImageBuffer::from_raw(image_data.width, image_data.height, 
                image_data.pixels.clone()
                .chunks(4)
                .map(|c| unsafe { std::mem::transmute::<[u8; 4], f32>(c.try_into().unwrap()) })
                .collect())
                .expect("doesn't fit??")
        ),
        _ => { panic!("different image format??"); },
    };
    let image = dyn_image.to_rgb32f();

    (vec![image.into()], Some(coords))
}