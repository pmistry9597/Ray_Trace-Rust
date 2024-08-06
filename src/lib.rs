use std::thread;
use std::sync::Arc;
use egui::mutex::Mutex;
use scene::Scene;
use std::sync::mpsc::{channel, Sender, Receiver};
pub use render::{RenderInfo, RenderTarget};
pub use builder::Scheme;

mod scene;
mod render;
mod ray;
mod material;
mod builder;
mod elements;
mod accel;

pub type ArcMux<T> = Arc<Mutex<T>>;
pub type BufferMux = Arc<Mutex<Vec<u8>>>;

const EPS: f32 = 1e-4;

pub struct RenderOut {
    pub buf_q: Receiver<Vec<u8>>,
}

pub struct Renderer {
    target: RenderTarget,
    sender: Sender<Vec<u8>>,

    scheme: Option<Scheme>,
}

impl Renderer {
    pub fn new(canv_width: i32, canv_height: i32, scheme: Scheme) -> (Self, RenderOut) {
        let buf: Vec<u8> = [0, 0, 0, 0].repeat((canv_width * canv_height).try_into().unwrap());
        let (tx, rx) = channel();
        let target = RenderTarget {
            buff_mux: Arc::new(Mutex::new(buf)),
            canv_width, canv_height,
        };
        (Self {
            target,
            sender: tx,
            scheme: Some(scheme),
        }, RenderOut{buf_q: rx})
    }
    pub fn consume_and_do(mut self) {
        thread::spawn(move || {
            let Scheme {
                cam, render_info, scene_members,
                ..
            } = self.scheme.take().unwrap();
            
            let skene = Scene { cam: cam.into(), members: scene_members.into() };

            render::render_to_target(&self.target, &skene, || self.update_output(), &render_info);
        });
    }

    fn update_output(&self) {
        self.sender.send(self.target.buff_mux.lock().clone()).expect("cannot send??");
    }
}

use std::cell::RefCell;
use rand::rngs::ThreadRng;

thread_local! {
    pub static RNG: RefCell<ThreadRng> = rand::thread_rng().into();
}