use nalgebra::Vector3;
use serde::Deserialize;
use crate::elements::mesh::Mesh;
use image::{DynamicImage, ImageBuffer};
use nalgebra::Vector2;

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

        let pbr_met_rough = primitive.material().pbr_metallic_roughness();
        let tex_info = pbr_met_rough.base_color_texture().expect("no base color texture??");
        let tex_data = &images[tex_info.texture().index()];

        let dyn_image = match tex_data.format {
            gltf::image::Format::R8G8B8 => DynamicImage::ImageRgb8(
                ImageBuffer::from_raw(tex_data.width, tex_data.height, tex_data.pixels.clone()).expect("doesn't fit??")
            ),
            _ => { panic!("different image format??"); },
        };
        let tex_image = dyn_image.to_rgb32f();

        let norm_info = primitive.material().normal_texture().expect("no normal map texture??");
        let norm_data = &images[norm_info.texture().index()];
        let dyn_norm = match norm_data.format {
            gltf::image::Format::R8G8B8 => DynamicImage::ImageRgb8(
                ImageBuffer::from_raw(norm_data.width, norm_data.height, norm_data.pixels.clone()).expect("doesn't fit??")
            ),
            _ => { panic!("different image format??"); },
        };
        let norm_image = dyn_norm.to_rgb32f();

        let tex_coords: Vec<Vector2<f32>> = reader.read_tex_coords(tex_info.tex_coord()).expect("no texture coordinates?").into_f32().map(|p| p.into()).collect();
        let norm_coords: Vec<Vector2<f32>> = reader.read_tex_coords(norm_info.tex_coord()).expect("no normal coordinates?").into_f32().map(|p| p.into()).collect();
        let tangents: Option<Vec<[f32; 3]>> = match reader.read_tangents() { 
            Some(it) => Some(it.map(|t| t[..3].try_into().unwrap()).collect()),
            None => None,
        };

        Mesh {
            poses,
            norms: reader.read_normals().unwrap().map(|p| p.into()).collect(),
            indices: flat_indices.chunks(3).map(|c| c.try_into().unwrap()).collect(),
            tex_coords,
            norm_coords,
            tangents: match tangents { Some(t) => Some(t.iter().map(|ta| (*ta).into()).collect()), None => None }, 
            textures: vec![tex_image.into()],
            normal_maps: vec![norm_image.into()],
        }
    }
}