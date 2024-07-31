use std::thread;
use std::sync::Arc;
use egui::mutex::Mutex;
use scene::Scene;
pub use render::{RenderInfo, RenderTarget};
pub use builder::Scheme;

mod scene;
mod render;
mod ray;
mod material;
mod builder;
mod elements;

pub type ArcMux<T> = Arc<Mutex<T>>;
pub type BufferMux = Arc<Mutex<Vec<u8>>>;

const EPS: f32 = 1e-4;

pub struct RenderOut {
    pub buffer_avail: ArcMux<Option<BufferMux>>,
}

pub struct Renderer {
    target: RenderTarget,
    out: Arc<RenderOut>,

    scheme: Option<Scheme>,
}

impl Renderer {
    pub fn new(canv_width: i32, canv_height: i32, scheme: Scheme) -> Self {
        let buf: Vec<u8> = [1, 0, 0, 0].repeat((canv_width * canv_height).try_into().unwrap());
        let target = RenderTarget {
            buff_mux: Arc::new(Mutex::new(buf)),
            canv_width, canv_height,
        };
        let buffer_avail = Arc::new(Mutex::new(Some(target.buff_mux.clone())));
        Self {
            target,
            out: Arc::new(RenderOut{ buffer_avail }),
            scheme: Some(scheme),
        }
    }
    pub fn get_out(&self) -> Arc<RenderOut> {
        self.out.clone()
    }
    pub fn consume_and_do(mut self) {
        thread::spawn(move || {
            let Scheme {
                cam, render_info, scene_elems
            } = self.scheme.take().unwrap();
            
            // ------------ cube map insertcion test ------------
            let mut skene = Scene { cam, elems: scene_elems.into() };
            use crate::elements::distant_cube_map::DistantCubeMap;
            skene.elems.push(Box::new(DistantCubeMap{}));
            // ------------ ------------ ------------ ------------

            render::render_to_target(&self.target, &skene, || self.update_output(), &render_info);
            // self.update_output();
            // thread::sleep(std::time::Duration::from_millis(500));
        });
    }

    fn update_output(&self) {
        *self.out.buffer_avail.lock() = Some(self.target.buff_mux.clone());
    }
}