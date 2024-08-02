use nalgebra::Vector3;
use crate::ray::{Ray, Hitable, HitResult, HitInfo, HasHitInfo, InteractsWithRay, DLSEmitter};
use crate::elements::IsCompleteElement;
use serde::Deserialize;

// --- -------- ------- - -- ----- - ----- FUCK --------------
// this file should be deleted/moved around soon!!
// --- --- --- --PEE ----- --- ----- ----

fn diagnostics(nfm: &MeshFromNode) {
    let (document, buffers, images) = gltf::import(&nfm.path).unwrap();
    let node_oi = document.nodes().nth(nfm.node_index).unwrap();

    println!("node of interest!!!!!!");
    println!("{} index: {}\n\n", node_oi.name().unwrap(), node_oi.index());

    let mesh = node_oi.mesh().unwrap();
    let primitives = mesh.primitives();

    println!("primitive count: {:?}", primitives.clone().count());

    // let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()].0));
}

#[derive(Deserialize, Debug)]
pub struct MeshFromNode {
    path: String,
    node_index: usize,
}
impl MeshFromNode {
    pub fn diagnostics(&self) { diagnostics(self); }
}

// --- -------- ------- POO POO POO  - -- ----- - -----
// --- -------- ------- POO POO POO  - -- ----- - -----
// dummy element section

pub struct DummyElement {}

impl IsCompleteElement for DummyElement {}

impl InteractsWithRay for DummyElement {
    fn continue_ray(&self, _ray: &Ray, _info: &HitInfo) -> Option<(Vector3<f32>, Ray)> {
        panic!("wtf r u doing here?");
    }
    fn give_dls_emitter(&self) -> Option<Box<dyn DLSEmitter + '_>> {
        panic!("wtf r u doing here?");
    }
}

impl HasHitInfo for DummyElement {
    fn hit_info(&self, _info: &HitResult, _ray: &Ray) -> HitInfo {
        panic!("wtf r u doing here?");
    }
}

impl Hitable for DummyElement {
    fn intersect(&self, _ray: &Ray) -> Option<HitResult> { None }
}