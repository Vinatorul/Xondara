use gfx;
use gfx::traits::FactoryExt;
use gfx::handle::ShaderResourceView;
use level;
use pipeline::{pipe, ColorFormat, DepthFormat};

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

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
               fragment_shader: &'static [u8],
               client_width: usize,
               client_height: usize) 
            -> Visualizer<R, F> {
        let pso = factory.create_pipeline_simple(
            verex_shader,
            fragment_shader,
            pipe::new()
        ).unwrap();

        let level = level::Level::new(&mut factory);
        let (vertex_data, index_data) = level.generate_mesh(client_width, client_height);
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
        
        let sampler = factory.create_sampler_linear();
        let data = pipe::Data {
            vbuf: vertex_buffer,
            out: main_color,
            texture: (level.texture, sampler),
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
