use nalgebra::{Vector3, Vector2, Matrix3, Matrix3x2, Matrix2};
use crate::elements::triangle::{Triangle, GimmeNorm, GimmeRgb};
use super::Mesh;
use std::ops::Index;
use std::iter::zip;

pub type MeshTriangle<'a> = Triangle<VertexFromMesh<'a>, NormFromMesh<'a>, RgbFromMesh<'a>>;

pub struct VertexFromMesh<'m> {
    pub index: (usize, usize),
    pub mesh: &'m Mesh,
}

impl Index<usize> for VertexFromMesh<'_> {
    type Output = Vector3<f32>;

    fn index(&self, vert_idx: usize) -> &Vector3<f32> {
        let (_prim_idx, inner_idx) = self.index;
        &self.mesh.poses[self.mesh.indices[inner_idx][vert_idx]]
    }
}

pub enum NormType {
    Mapped{tang_to_mod: Matrix3<f32>},
    Uniform(Vector3<f32>),
}

pub struct NormFromMesh<'m> {
    pub index: (usize, usize),
    pub norm_type: NormType,
    pub mesh: &'m Mesh,
}
impl<'m> NormFromMesh<'m> {
    pub fn from_mesh_and_inner_idx(mesh: &'m Mesh, full_idx: (usize, usize)) -> Self {
        NormFromMesh {
            index: full_idx,
            norm_type: Self::generate_norm_type(mesh, full_idx),
            mesh,
        }
    }

    fn generate_norm_type(mesh: &Mesh, full_idx: (usize, usize)) -> NormType {
        // some help from https://www.opengl-tutorial.org/intermediate-tutorials/tutorial-13-normal-mapping/
        // to get the tangent to model space transform

        let (_prim_idx, inner_idx) = full_idx;
        let indices = &mesh.indices[inner_idx];
        let face_norm = Self::get_face_norm(mesh, full_idx);

        use NormType::*;
        match &mesh.tangents {
            // _ => Uniform(face_norm),

            Some(tans) => {
                // maybe we need to fix below calculation
                // of tangent vector, might mess up tan -> mod transform for normal maps
                let tan: Vector3<f32> = mesh.indices[inner_idx].iter()
                    .map(|i| tans[*i])
                    .sum();
                let tan = tan.normalize();
                let bitan = tan.cross(&face_norm);

                Mapped{tang_to_mod: Matrix3::from_columns(&[tan.normalize(), bitan.normalize(), face_norm])}
            },
            None => {
                let t1 = mesh.tex_coords[indices[1]] - mesh.tex_coords[indices[0]];
                let t2 = mesh.tex_coords[indices[2]] - mesh.tex_coords[indices[0]];

                let tex_poses = Matrix2::from_columns(&[t1, t2]);
                match tex_poses.try_inverse() {
                    Some(inv_tex_poses) => {
                        let e1 = mesh.poses[indices[1]] - mesh.poses[indices[0]];
                        let e2 = mesh.poses[indices[2]] - mesh.poses[indices[0]];
                        
                        let mod_poses = Matrix3x2::from_columns(&[e1, e2]);
                        let incomplete = mod_poses * inv_tex_poses; // gives T and B as its columns

                        let mut tang_to_mod: Matrix3<f32> = incomplete.fixed_resize(0.0);
                        for i in 0..2 {
                            tang_to_mod.set_column(i, &tang_to_mod.column(i).normalize());
                        }
                        tang_to_mod.set_column(2, &face_norm);

                        Mapped{tang_to_mod}
                    },
                    None => Uniform(face_norm),
                }
            }
        }
    }

    fn get_face_norm(mesh: &Mesh, full_idx: (usize, usize)) -> Vector3<f32> {
        let (_prim_idx, inner_idx) = full_idx;
        let cum: Vector3<f32> = mesh.indices[inner_idx].iter()
            .map(|i| mesh.norms[*i])
            .sum();
        cum.normalize()
    }
}

impl GimmeNorm for NormFromMesh<'_> {
    fn get_norm(&self, barycentric: &(f32, f32)) -> Vector3<f32> {
        use NormType::*;
        match self.norm_type {
            Mapped{tang_to_mod} => {
                let (b1, b2) = *barycentric;
                let b0 = 1.0 - b2 - b1;
                let baryc: [f32; 3] = [b0, b1, b2];

                let (prim_idx, inner_idx) = self.index;
                let tex_coord: Vector2<f32> = zip(self.mesh.indices[inner_idx].iter(), baryc.iter())
                    .map(|(i, b)| self.mesh.tex_coords[*i] * *b)
                    .sum();

                let norm = tang_to_mod * self.mesh.normal_maps[prim_idx].get_pixel(tex_coord.x, tex_coord.y);
                norm.normalize()
            },
            Uniform(norm) => norm,
        }
    }
}

pub struct RgbFromMesh<'m> {
    pub index: (usize, usize),
    pub mesh: &'m Mesh,
}

impl GimmeRgb for RgbFromMesh<'_> {
    fn get_rgb(&self, barycentric: &(f32, f32)) -> Vector3<f32> {
        let (b1, b2) = *barycentric;
        let b0 = 1.0 - b2 - b1;
        let baryc: [f32; 3] = [b0, b1, b2];

        let (prim_idx, inner_idx) = self.index;
        let tex_coord: Vector2<f32> = zip(self.mesh.indices[inner_idx].iter(), baryc.iter())
            .map(|(i, b)| self.mesh.tex_coords[*i] * *b)
            .sum();
        
        self.mesh.textures[prim_idx].get_pixel(tex_coord.x, tex_coord.y)
    }
}