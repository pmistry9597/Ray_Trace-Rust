use std::thread;
use std::sync::Arc;
use egui::mutex::Mutex;

mod scene;

pub type ArcMux<T> = Arc<Mutex<T>>;
pub type BufferMux = Arc<Mutex<Vec<u8>>>;

const EPS: f32 = 1e-4;

struct RenderTarget {
    buff_mux: BufferMux,
    canv_width: i32, canv_height: i32,
}
impl RenderTarget {
    // fn pix_to_idx(&self, x: i32, y: i32) -> usize {
    //     let idx = (y * self.canv_width + x) * 4;
    //     idx.try_into().unwrap()
    // }
    // fn pix_idx_vec(&self) -> Vec<((i32, i32), usize)> {
    //     (0..self.canv_width).flat_map(|x| (0..self.canv_height).map( move |y| ((x, y), self.pix_to_idx(x, y)) ))
    //     .collect()
    // }

    // fn idx_to_pix(&self, idx: i32) -> (i32, i32) {
    //     let pix_adj = idx / 4;
    //     let x = pix_adj % self.canv_width;
    //     let y = (pix_adj - x) / self.canv_width;
    //     (x.try_into().unwrap(), y.try_into().unwrap())
    // }
    fn chunk_to_pix(&self, idx: i32) -> (i32, i32) {
        // let pix_adj = idx / 4;
        let x = idx % self.canv_width;
        let y = (idx - x) / self.canv_width;
        (x.try_into().unwrap(), y.try_into().unwrap())
    }
}

pub struct Renderer {
    render_target: RenderTarget,
    buffer_avail: ArcMux<Option<BufferMux>>,
}

impl Renderer {
    pub fn new(canv_width: i32, canv_height: i32,) -> Self {
        let buf: Vec<u8> = [1, 0, 0, 0].repeat((canv_width * canv_height).try_into().unwrap());
        let render_target = RenderTarget{
            buff_mux: Arc::new(Mutex::new(buf)),
            canv_width, canv_height,
        };
        let buffer_avail = Arc::new(Mutex::new(Some(render_target.buff_mux.clone())));
        Self {
            render_target,
            buffer_avail,
        }
    }
    pub fn get_buffer_avail(&self) -> ArcMux<Option<BufferMux>> {
        self.buffer_avail.clone()
    }
    pub fn consume_and_do(self) {
        thread::spawn(move || {
            
            let mut skene = scene::give_crap();
            use nalgebra::{vector, Vector3};
            skene.cam.d = vector![0.5, 0.0, -5.0];
            let mut other_d: Vector3<f32> = vector![-0.5, 0.0, -5.0];

            loop {
                std::mem::swap(&mut skene.cam.d, &mut other_d);
                scene::render_to_target(&self.render_target, &skene);
                self.declare_avail();
                thread::sleep(std::time::Duration::from_millis(500));

            }
        });
    }

    fn declare_avail(&self) {
        *self.buffer_avail.lock() = Some(self.render_target.buff_mux.clone());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_renderer_chunk_to_pix() {
        let (canv_width, canv_height) = (400, 500);
        let render_target = RenderTarget { 
            buff_mux: Arc::new(Mutex::new(vec![])), 
            canv_width, canv_height 
        };

        let begin_idx = 0;
        let expected_begin = (0, 0);
        assert_eq!(render_target.chunk_to_pix(begin_idx), expected_begin, "Begin index wrong");

        let bot_right_idx = canv_width - 1;
        let expected_bot_right = (canv_width - 1, 0);
        assert_eq!(render_target.chunk_to_pix(bot_right_idx), expected_bot_right, "Bottom right index wrong");

        let top_right_idx = (canv_height * canv_width - 1);
        let expected_top_right = (canv_width - 1, canv_height - 1);
        assert_eq!(render_target.chunk_to_pix(top_right_idx), expected_top_right, "Top right index wrong");

        let top_left_idx = (canv_height - 1) * canv_width;
        let expected_top_left = (0, canv_height - 1);
        assert_eq!(render_target.chunk_to_pix(top_left_idx), expected_top_left, "Top left index wrong");
    }
}