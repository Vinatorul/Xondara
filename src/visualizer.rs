use gfx;
use gfx::traits::FactoryExt;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;


const TRIANGLE: [Vertex; 3] = [
    Vertex { pos: [ -0.5, -0.5 ], color: [1.0, 0.0, 0.0] },
    Vertex { pos: [  0.5, -0.5 ], color: [0.0, 1.0, 0.0] },
    Vertex { pos: [  0.0,  0.5 ], color: [0.0, 0.0, 1.0] }
];

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    pipeline pipe {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
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
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&TRIANGLE, ());
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: main_color
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