use gfx;
use gfx::traits::FactoryExt;
use level;
use pipeline::{pipe, ColorFormat, DepthFormat};

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

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
               vertex_shader: &'static [u8],
               fragment_shader: &'static [u8],
               client_width: usize,
               client_height: usize) 
            -> Visualizer<R, F> {
        let program = factory.link_program(&vertex_shader, &fragment_shader).unwrap();
        let mut rasterizer = gfx::state::Rasterizer::new_fill();
        rasterizer.method = gfx::state::RasterMethod::Line(10);
        let pso = factory.create_pipeline_from_program(
            &program,
            gfx::Primitive::LineList,
            rasterizer,
            pipe::new()
        ).unwrap();

        let level = level::Level::new();
        let (vertex_data, index_data) = level.generate_mesh();
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
        
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: main_color,
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
