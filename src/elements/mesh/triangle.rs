use nalgebra::{Vector3, Vector2};
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

pub struct NormFromMesh<'m> {
    pub index: (usize, usize),
    pub mesh: &'m Mesh,
}

impl GimmeNorm for NormFromMesh<'_> {
    fn get_norm(&self, _pos: &Vector3<f32>) -> Vector3<f32> {
        // temporary, get the avg of all norms
        // later may use normal mapping
        let (_prim_idx, inner_idx) = self.index;
        let cum: Vector3<f32> = self.mesh.indices[inner_idx].iter()
            .map(|i| self.mesh.norms[*i])
            .sum();
        cum.normalize()
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