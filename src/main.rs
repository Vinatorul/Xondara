#[macro_use]
extern crate glium;
extern crate image;

mod visualizer;

use glium::{DisplayBuild, Surface};
use visualizer::Visualizer;
use std::thread;
use std::time::Duration;

fn main() {
    
    let display = glium::glutin::WindowBuilder::new()
        .with_title("Xondara".to_string())
        .build_glium()
        .unwrap();    

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        uniform mat4 matrix;
        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        uniform sampler2D tex;
        void main() {
            gl_FragColor = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    let mut visualizer = Visualizer::new(display, program);

    loop {
        visualizer.draw();
        visualizer.proc_event();   
        thread::sleep(Duration::from_millis(10));     
    }
}
