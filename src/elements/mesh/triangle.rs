use nalgebra::{Vector3, Vector2, Matrix3, Matrix3x2, Matrix2};
use crate::elements::triangle::{Triangle, GimmeNorm, GimmeRgb, DivertsRay};
use crate::ray::Ray;
use super::Mesh;
use std::ops::Index;
use std::iter::zip;
use crate::material::DynDiffSpec;

pub type MeshTriangle<'a> = Triangle<VertexFromMesh<'a>, NormFromMesh<'a>, RgbFromMesh<'a>, DivertsRayFromMesh<'a>>;

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
                let (prim_idx, _inner_idx) = self.index;
                let norm_coord = tex_coord_from_bary(self.mesh, &self.mesh.norm_coords, barycentric, self.index);

                let norm = tang_to_mod * self.mesh.normal_maps[prim_idx].get_pixel(norm_coord.x, norm_coord.y);
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
        let (prim_idx, _inner_idx) = self.index;
        let tex_coord = tex_coord_from_bary(self.mesh, &self.mesh.tex_coords, barycentric, self.index);
        
        self.mesh.textures[prim_idx].get_pixel(tex_coord.x, tex_coord.y)
    }
}

pub struct DivertsRayFromMesh<'m> {
    pub index: (usize, usize),
    pub mesh: &'m Mesh,
}

impl DivertsRay for DivertsRayFromMesh<'_> {
    type Seeding = (bool, f32); // (should_diff, roughness)

    fn divert_ray_seed(&self, ray: &Ray, norm: &Vector3<f32>, barycentric: &(f32, f32)) -> Self::Seeding {
        let (prim_idx, _inner_idx) = self.index;

        let (metalness, roughness) = match &self.mesh.metal_rough.coords {
            Some(coords) => {
                let mr_coord = tex_coord_from_bary(self.mesh, coords, barycentric, self.index);
                let mr_val = self.mesh.metal_rough_maps[prim_idx].get_pixel(mr_coord.x, mr_coord.y);
                (mr_val[2] * self.mesh.metal_rough.metal, mr_val[1] * self.mesh.metal_rough.rough)
            },
            None => (self.mesh.metal_rough.metal, self.mesh.metal_rough.rough),
        };
        
        const CUSTOM_ATTEN: f32 = 1.0; // attenuate metal because i think model didnt expect reflections!
        let r0 = 0.04 + (1.0 - 0.04) * metalness; // based on gltf definition of metalness for fresnel
        let reflectance = r0 + (1.0 - r0) * CUSTOM_ATTEN * (1.0 - (ray.d.dot(&norm)).abs().powf(5.0)); // schlick approximation

        // DynDiffSpec::should_diff(1.0 - reflectance)
        (DynDiffSpec::should_diff(1.0 - reflectance), roughness)
    }

    fn divert_new_ray(&self, ray: &Ray, norm: &Vector3<f32>, o: &Vector3<f32>, seeding: &Self::Seeding) -> (Ray, f32) {
        let (should_diff, roughness) = *seeding;
        let (mut ray, p) = DynDiffSpec::gen_new_ray(ray, norm, o, should_diff);

        // we do roughness here, modify the ray
        let scatter: Vector3<f32> = {
            use rand::Rng;
            use nalgebra::vector;
            let mut rng = rand::thread_rng();
            let u: f32 = rng.gen();
            let v: f32 = rng.gen();
            let w: f32 = rng.gen();
            roughness * vector![u,v,w].normalize()
        };

        ray.d = (ray.d + scatter).normalize();
        (ray, p)
    }
}

fn tex_coord_from_bary(mesh: &Mesh, coords: &Vec<Vector2<f32>>, barycentric: &(f32, f32), full_idx: (usize, usize)) -> Vector2<f32> {
    let (b1, b2) = *barycentric;
    let b0 = 1.0 - b2 - b1;
    let baryc: [f32; 3] = [b0, b1, b2];

    let (_prim_idx, inner_idx) = full_idx;
    zip(mesh.indices[inner_idx].iter(), baryc.iter())
        .map(|(i, b)| coords[*i] * *b)
        .sum()
}