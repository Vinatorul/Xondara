use gfx;
use gfx::traits::FactoryExt;
use gfx::{Bundle, tex};
use level;
use pipeline::{scene, result, ColorFormat, DepthFormat, ResultVertex};

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Visualizer<R> where R: gfx::Resources {
    scene: Bundle<R, scene::Data<R>>,
    result: Bundle<R, result::Data<R>>,
}

struct ViewPair<R: gfx::Resources, T: gfx::format::Formatted> {
    resource: gfx::handle::ShaderResourceView<R, T::View>,
    target: gfx::handle::RenderTargetView<R, T>,
}

impl<R> Visualizer<R> where R: gfx::Resources {
    pub fn new<F>(mut factory: F, 
               main_color: gfx::handle::RenderTargetView<R, ColorFormat>,
               _main_depth: gfx::handle::DepthStencilView<R, DepthFormat>,
               vertex_shader: &'static [u8],
               fragment_shader: &'static [u8],
               result_vertex_shader: &'static [u8],
               result_fragment_shader: &'static [u8])
            -> Visualizer<R> where F: gfx::Factory<R>
    {

        let (width, height, _, _) = main_color.get_dimensions();
        let hdr_tex = {
            let (_ , srv, rtv) = factory.create_render_target(width, height).unwrap();
            ViewPair{ resource: srv, target: rtv }
        };

        let sampler = factory.create_sampler(
            tex::SamplerInfo::new(tex::FilterMethod::Scale,
                                       tex::WrapMode::Clamp)
        );

        let scene = {
            let level = level::Level::new();
            let (vertex_data, index_data) = level.generate_mesh();
            let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
            let program = factory.link_program(&vertex_shader, &fragment_shader).unwrap();
            let mut rasterizer = gfx::state::Rasterizer::new_fill();
            rasterizer.method = gfx::state::RasterMethod::Line(10);
            let pso = factory.create_pipeline_from_program(
                &program,
                gfx::Primitive::LineList,
                rasterizer,
                scene::new()
            ).unwrap();
            let data = scene::Data {
                vbuf: vertex_buffer,
                out: hdr_tex.target.clone(),
            };

            Bundle::new(slice, pso, data)
        };
        
        let result = {
            let vertex_data = [
                ResultVertex { pos: [ -1.0,  1.0 ], tex_coord: [0.0, 0.0] },
                ResultVertex { pos: [  1.0,  1.0 ], tex_coord: [1.0, 0.0] },
                ResultVertex { pos: [ -1.0, -1.0 ], tex_coord: [0.0, 1.0] },
                ResultVertex { pos: [  1.0, -1.0 ], tex_coord: [1.0, 1.0] },
            ];
            let index_data: Vec<u32> = vec![0, 1, 2, 1, 2, 3];
            let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
            let pso = factory.create_pipeline_simple(
                result_vertex_shader,
                result_fragment_shader,
                result::new()
                ).unwrap();
            let data = result::Data {
                vbuf: vertex_buffer,
                tex: (hdr_tex.resource.clone(), sampler.clone()),
                out: main_color,
            };
            Bundle::new(slice, pso, data)
        };
        
        
        Visualizer {
            scene: scene,
            result: result,
        }
    }

    pub fn render<C>(&mut self, encoder: &mut gfx::Encoder<R, C>) where C: gfx::CommandBuffer<R> {
        encoder.clear(&self.scene.data.out, CLEAR_COLOR);
        self.scene.encode(encoder);
        self.result.encode(encoder);
    }
}
