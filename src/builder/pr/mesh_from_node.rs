use nalgebra::Vector3;
use serde::Deserialize;
use crate::elements::mesh::{Mesh, PbrMetalRough};
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
        let (normal_maps, norm_coords) = norminfo_to_uvtex_and_coords(&material.normal_texture(), &reader, &images);

        let tangents: Option<Vec<[f32; 3]>> = reader.read_tangents().map(|tans| tans.map(|t| t[..3].try_into().unwrap()).collect());

        let (metal_rough_maps, mr_coords) = texinfo_to_uvtex_and_coords(&pbr_met_rough.metallic_roughness_texture(), &reader, &images);
        let metal_rough = PbrMetalRough {
            metal: pbr_met_rough.metallic_factor(),
            rough: pbr_met_rough.roughness_factor(),
            coords: mr_coords,
        };

        Mesh {
            poses,
            norms: reader.read_normals().unwrap().map(|p| p.into()).collect(),
            indices: flat_indices.chunks(3).map(|c| c.try_into().unwrap()).collect(),
            tex_coords: tex_coords.unwrap(),
            norm_coords: norm_coords.unwrap(),
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
use gltf::material::NormalTexture;
use gltf::{Buffer, Texture};

fn norminfo_to_uvtex_and_coords<'a, 's, F>(norm_info: &Option<NormalTexture>, reader: &Reader<'a, 's, F>, images: &Vec<Data>) -> (Vec<UVRgb32FImage>, Option<Vec<Vector2<f32>>>) 
where
    F: Clone + Fn(Buffer<'a>) -> Option<&'s [u8]>,
{
    match norm_info {
        Some(info) => get_uvtex_and_coords(&info.texture(), info.tex_coord(), reader, images),
        None => (vec![], None),
    }
}

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
    let dyn_image = match image_data.format {
        gltf::image::Format::R8G8B8 => DynamicImage::ImageRgb8(
            ImageBuffer::from_raw(image_data.width, image_data.height, image_data.pixels.clone()).expect("doesn't fit??")
        ),
        _ => { panic!("different image format??"); },
    };
    let image = dyn_image.to_rgb32f();

    (vec![image.into()], Some(coords))
}