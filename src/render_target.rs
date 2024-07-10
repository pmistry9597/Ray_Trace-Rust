use crate::BufferMux;

pub struct RenderTarget {
    pub buff_mux: BufferMux,
    pub canv_width: i32, pub canv_height: i32,
}
impl RenderTarget {
    pub fn chunk_to_pix(&self, idx: i32) -> (i32, i32) {
        // let pix_adj = idx / 4;
        let x = idx % self.canv_width;
        let y = (idx - x) / self.canv_width;
        (x.try_into().unwrap(), y.try_into().unwrap())
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