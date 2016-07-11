#[macro_use]
extern crate gfx;

extern crate gfx_window_glutin;
extern crate gfx_device_gl;
extern crate glutin;
extern crate image;

use gfx::Device;

mod visualizer;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let builder = glutin::WindowBuilder::new()
        .with_title("Xondara".to_string())
        .with_dimensions(1024, 768)
        .with_vsync();
    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<visualizer::ColorFormat, visualizer::DepthFormat>(builder);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut visualizer = visualizer::Visualizer::new(
        factory, 
        main_color, 
        main_depth,
        include_bytes!("shader/120_vert.glsl"),
        include_bytes!("shader/120_frag.glsl"));
    'main: loop {
        // loop over events
        for event in window.poll_events() {
            match event {
                glutin::Event::KeyboardInput(_, _, Some(glutin::VirtualKeyCode::Escape)) |
                glutin::Event::Closed => break 'main,
                _ => {},
            }
        }
        // draw a frame
        visualizer.render(&mut encoder);
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}