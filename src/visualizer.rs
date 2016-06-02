use std::io::Cursor;
use glium;
use glium::{Surface};
use image;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

pub struct Visualizer {
    display: glium::Display,
    program: glium::Program,
    texture: glium::Texture2d,
    vertex_buffer: glium::VertexBuffer<Vertex>,
    indices: glium::index::NoIndices,
}

impl Visualizer {
    pub fn new(display: glium::Display, program: glium::Program) -> Visualizer {
        let image = image::load(Cursor::new(&include_bytes!("../assets/test.png")[..]),
                            image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = glium::texture::Texture2d::new(&display, image).unwrap();        

        let vertex1 = Vertex { position: [ -0.5, 0.5], tex_coords: [0.0, 1.0] };
        let vertex2 = Vertex { position: [ 0.5, 0.5], tex_coords: [1.0, 1.0] };
        let vertex3 = Vertex { position: [ -0.5, -0.5], tex_coords: [0.0, 0.0] };
        let vertex4 = Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0] };
        let shape = vec![vertex1, vertex3, vertex2, vertex2, vertex3, vertex4];

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        Visualizer {
            display: display,
            program: program,
            texture: texture,
            vertex_buffer: vertex_buffer,
            indices: indices,
        }
    } 


    pub fn draw(&self) {
        let mut target = self.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniforms = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ],
            tex: &self.texture,
        };

        target.draw(&self.vertex_buffer, &self.indices, &self.program, &uniforms,
                    &Default::default()).unwrap();
        target.finish().unwrap();
    }

    pub fn proc_event(&mut self) {
        for ev in self.display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}