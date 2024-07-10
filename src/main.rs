use ray_trace_rust::{BufferMux, ArcMux, Renderer, RenderOut};
use ui_util::ui_on_render_out;

mod ui_util;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let (region_width, region_height) = (1200 as i32, 600 as i32);
    let buffer_renderer = Renderer::new(region_width, region_height);
    let render_out = buffer_renderer.get_out();
    buffer_renderer.consume_and_do();

    ui_on_render_out(render_out, (region_width, region_height))
}