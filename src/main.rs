use ray_trace_rust::{BufferMux, ArcMux, Renderer, RenderOut, Scheme};
use ui_util::ui_on_render_out;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

mod ui_util;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let paff = Path::new("/home/moistry/Documents/endeavors/raytracer_graphics_fuck/ray_trace-rust/scheme.yml");
    let mut file = File::open(&paff).expect("file boss???");
    let mut scheme_dat = String::new();
    file.read_to_string(&mut scheme_dat).unwrap();

    let scheme = Scheme::from_yml(scheme_dat);

    let (region_width, region_height) = (1200 as i32, 600 as i32);
    let buffer_renderer = Renderer::new(region_width, region_height, scheme);
    let render_out = buffer_renderer.get_out();
    buffer_renderer.consume_and_do();

    ui_on_render_out(render_out, (region_width, region_height))
}