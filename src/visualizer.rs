use std::io::Cursor;
use gfx;
use gfx::traits::FactoryExt;
use gfx::handle::ShaderResourceView;
use gfx::tex;
use image;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;


const TRIANGLE: [Vertex; 4] = [
    Vertex { pos: [ -0.5, -0.5 ], tc: [0.0, 1.0] },
    Vertex { pos: [  -0.5, 0.5 ], tc: [0.0, 0.0] },
    Vertex { pos: [  0.5, -0.5 ], tc: [1.0, 1.0] },
    Vertex { pos: [  0.5,  0.5 ], tc: [1.0, 0.0] }
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        tc: [f32; 2] = "a_Tc",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
        texture: gfx::TextureSampler<[f32; 4]> = "t_Test",
    }
}

fn load_texture<R, F>(factory: &mut F, data: &[u8]) -> ShaderResourceView<R, [f32; 4]>
    where R: gfx::Resources, F: gfx::Factory<R>
{
    let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = tex::Kind::D2(width as tex::Size, height as tex::Size, tex::AaMode::Single);
    let (_, view) = factory.create_texture_const_u8::<ColorFormat>(kind, &[&img]).unwrap();
    view
}

pub struct Visualizer<R, F> where R: gfx::Resources, F: gfx::Factory<R> {
    pso: gfx::PipelineState<R, pipe::Meta>,
    factory: F,
    data: pipe::Data<R>,
    slice: gfx::Slice<R>,
}

impl<R, F> Visualizer<R, F> where R: gfx::Resources, F: gfx::Factory<R> {
    pub fn new(mut factory: F, 
               main_color: gfx::handle::RenderTargetView<R, ColorFormat>,
               _main_depth: gfx::handle::DepthStencilView<R, DepthFormat>,
               verex_shader: &'static [u8],
               fragment_shader: &'static [u8]) 
            -> Visualizer<R, F> {
        let pso = factory.create_pipeline_simple(
            verex_shader,
            fragment_shader,
            pipe::new()
        ).unwrap();
        let index_buffer: &[u16] = &[0,  1,  2,  1,  2,  3];
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, index_buffer);
        let texture_data = &include_bytes!("../assets/test.png")[..];
        let test_texture = load_texture(&mut factory, texture_data);
        let sampler = factory.create_sampler_linear();
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: main_color,
            texture: (test_texture, sampler),
        };
        Visualizer {
            pso: pso,
            factory: factory,
            data: data,
            slice: slice,
        }
    }

    pub fn render<C>(&mut self, encoder: &mut gfx::Encoder<R, C>) where C: gfx::CommandBuffer<R> {
        encoder.clear(&self.data.out, CLEAR_COLOR);
        encoder.draw(&self.slice, &self.pso, &self.data);
    }
}
