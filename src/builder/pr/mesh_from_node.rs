use nalgebra::Vector3;
use serde::Deserialize;
use crate::elements::mesh::Mesh;
use image::{DynamicImage, ImageBuffer};

// --- -------- ------- - -- ----- - ----- FUCK --------------
// this file should be deleted/changed around soon!!
// --- --- --- --PEE ----- --- ----- ----

#[derive(Deserialize, Debug)]
pub struct MeshFromNode {
    path: String,
    node_index: usize,
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
        let poses: Vec<Vector3<f32>> = poses.iter().map(|v| v * 0.05).collect();

        let pbr_met_rough = primitive.material().pbr_metallic_roughness();
        let texture = pbr_met_rough.base_color_texture().expect("no base color texture??").texture();
        let tex_data = &images[texture.index()];
        // println!("texture data format: {:?}", tex_data.format);
        let dyn_image = match tex_data.format {
            gltf::image::Format::R8G8B8 => DynamicImage::ImageRgb8(
                ImageBuffer::from_raw(tex_data.width, tex_data.height, tex_data.pixels.clone()).expect("doesn't fit??")
            ),
            _ => { panic!("different image format??"); },
        };
        let tex_image = dyn_image.to_rgb32f();

        Mesh {
            poses,
            norms: reader.read_normals().unwrap().map(|p| p.into()).collect(),
            indices: flat_indices.chunks(3).map(|c| c.try_into().unwrap()).collect(),
            tex_coords: reader.read_tex_coords(0).unwrap().into_f32().map(|p| p.into()).collect(),
            textures: vec![tex_image.into()],
        }
    }
}