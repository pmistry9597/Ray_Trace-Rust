use std::thread;
use std::sync::Arc;
use egui::mutex::Mutex;
use render_target::RenderTarget;
pub use scene::RenderInfo;
pub use builder::Scheme;

mod scene;
mod render_target;
mod ray;
mod material;
mod builder;

pub type ArcMux<T> = Arc<Mutex<T>>;
pub type BufferMux = Arc<Mutex<Vec<u8>>>;

const EPS: f32 = 1e-4;

pub struct RenderOut {
    pub buffer_avail: ArcMux<Option<BufferMux>>,
}

struct ConsumeOnRun {
    render_info: RenderInfo,
    cam: scene::Cam,
}

pub struct Renderer {
    target: RenderTarget,
    out: Arc<RenderOut>,

    consume_on_run: Option<ConsumeOnRun>,
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

            consume_on_run: Some(ConsumeOnRun {
                render_info: scheme.render_info,
                cam: scheme.cam,
            }),
        }
    }
    pub fn get_out(&self) -> Arc<RenderOut> {
        self.out.clone()
    }
    pub fn consume_and_do(mut self) {
        thread::spawn(move || {
            let mut skene = scene::test_rig::walled();
            // skene.cam = self.cam;
            let ConsumeOnRun{
                cam, render_info
            } = self.consume_on_run.take().unwrap();
            skene.cam = cam;

            use nalgebra::{vector, Vector3};
            skene.cam.d = vector![0.5, 0.0, -5.0];
            let mut other_d: Vector3<f32> = vector![-0.5, 0.0, -5.0];

            loop {
                std::mem::swap(&mut skene.cam.d, &mut other_d);
                scene::render_to_target(&self.target, &skene, || self.update_output(), &render_info);
                // self.update_output();
                thread::sleep(std::time::Duration::from_millis(500));

            }
        });
    }

    fn update_output(&self) {
        *self.out.buffer_avail.lock() = Some(self.target.buff_mux.clone());
    }
}