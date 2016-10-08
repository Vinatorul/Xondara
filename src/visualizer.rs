use gfx;
use gfx::traits::FactoryExt;
use gfx::{Bundle, tex};
use gfx::handle::{ShaderResourceView, RenderTargetView};
use level;
use common::*;
use pipeline::{scene, result, blur, ColorFormat, HDRFormat, DepthFormat, PPVertex};

const CLEAR_COLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct Visualizer<R>
    where R: gfx::Resources
{
    scene: Bundle<R, scene::Data<R>>,
    result: Bundle<R, result::Data<R>>,
    blur: Bundle<R, blur::Data<R>>,
    source: gfx::handle::ShaderResourceView<R, [f32; 4]>,
    pair1: ViewPair<R, HDRFormat>,
    pair2: ViewPair<R, HDRFormat>,
}

struct ViewPair<R: gfx::Resources, T: gfx::format::Formatted> {
    resource: ShaderResourceView<R, T::View>,
    target: RenderTargetView<R, T>,
}

impl<R> Visualizer<R>
    where R: gfx::Resources
{
    pub fn new<F>(mut factory: F,
                  main_color: gfx::handle::RenderTargetView<R, ColorFormat>,
                  _main_depth: gfx::handle::DepthStencilView<R, DepthFormat>)
                  -> Visualizer<R>
        where F: gfx::Factory<R>
    {

        let (width, height, _, _) = main_color.get_dimensions();
        let hdr_tex1 = {
            let (_, srv, rtv) = factory.create_render_target(width, height).unwrap();
            ViewPair {
                resource: srv,
                target: rtv,
            }
        };
        let hdr_tex2 = {
            let (_, srv, rtv) = factory.create_render_target(width, height).unwrap();
            ViewPair {
                resource: srv,
                target: rtv,
            }
        };

        let hdr_tex3 = {
            let (_, srv, rtv) = factory.create_render_target(width, height).unwrap();
            ViewPair {
                resource: srv,
                target: rtv,
            }
        };

        let sampler =
            factory.create_sampler(tex::SamplerInfo::new(tex::FilterMethod::Scale,
                                                         tex::WrapMode::Clamp));

        let scene = {
            let level = level::Level::new();
            let (vertex_data, index_data) = level.generate_mesh();
            let (vertex_buffer, slice) =
                factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
            let program = factory.link_program(vertex_shader(), fragment_shader()).unwrap();
            let mut rasterizer = gfx::state::Rasterizer::new_fill();
            rasterizer.method = gfx::state::RasterMethod::Line(5);
            let pso = factory.create_pipeline_from_program(&program,
                                              gfx::Primitive::LineList,
                                              rasterizer,
                                              scene::new())
                .unwrap();
            let data = scene::Data {
                vbuf: vertex_buffer,
                out: hdr_tex1.target.clone(),
            };

            Bundle::new(slice, pso, data)
        };

        let result = {
            let vertex_data = [PPVertex {
                                   pos: [-1.0, 1.0],
                                   tex_coord: [0.0, 0.0],
                               },
                               PPVertex {
                                   pos: [1.0, 1.0],
                                   tex_coord: [1.0, 0.0],
                               },
                               PPVertex {
                                   pos: [-1.0, -1.0],
                                   tex_coord: [0.0, 1.0],
                               },
                               PPVertex {
                                   pos: [1.0, -1.0],
                                   tex_coord: [1.0, 1.0],
                               }];
            let index_data: Vec<u32> = vec![0, 1, 2, 1, 2, 3];
            let (vertex_buffer, slice) =
                factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
            let pso = factory.create_pipeline_simple(pp_vertex_shader(),
                                        result_fragment_shader(),
                                        result::new())
                .unwrap();
            let data = result::Data {
                vbuf: vertex_buffer,
                scene: (hdr_tex1.resource.clone(), sampler.clone()),
                blur: (hdr_tex3.resource.clone(), sampler.clone()),
                out: main_color,
            };
            Bundle::new(slice, pso, data)
        };

        let blur = {
            let vertex_data = [PPVertex {
                                   pos: [-1.0, 1.0],
                                   tex_coord: [0.0, 0.0],
                               },
                               PPVertex {
                                   pos: [1.0, 1.0],
                                   tex_coord: [1.0, 0.0],
                               },
                               PPVertex {
                                   pos: [-1.0, -1.0],
                                   tex_coord: [0.0, 1.0],
                               },
                               PPVertex {
                                   pos: [1.0, -1.0],
                                   tex_coord: [1.0, 1.0],
                               }];
            let index_data: Vec<u32> = vec![0, 1, 2, 1, 2, 3];
            let (vertex_buffer, slice) =
                factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);
            let pso =
                factory.create_pipeline_simple(pp_vertex_shader(),
                                            blur_fragment_shader(),
                                            blur::new())
                    .unwrap();
            let data = blur::Data {
                vbuf: vertex_buffer,
                direction: 0,
                tex: (hdr_tex1.resource.clone(), sampler.clone()),
                out: hdr_tex2.target.clone(),
            };
            Bundle::new(slice, pso, data)
        };


        Visualizer {
            scene: scene,
            result: result,
            blur: blur,
            source: hdr_tex1.resource.clone(),
            pair1: hdr_tex2,
            pair2: hdr_tex3,
        }
    }

    pub fn render<C>(&mut self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>
    {
        encoder.clear(&self.scene.data.out, CLEAR_COLOR);
        self.scene.encode(encoder);
        self.blur.data.tex.0 = self.source.clone();
        self.blur.data.out = self.pair1.target.clone();
        self.blur.data.direction = 0;
        self.blur.encode(encoder);
        self.blur.data.tex.0 = self.pair1.resource.clone();
        self.blur.data.out = self.pair2.target.clone();
        self.blur.data.direction = 1;
        self.blur.encode(encoder);
        for _i in 0..1 {
            self.blur.data.tex.0 = self.pair2.resource.clone();
            self.blur.data.out = self.pair1.target.clone();
            self.blur.data.direction = 0;
            self.blur.encode(encoder);
            self.blur.data.tex.0 = self.pair1.resource.clone();
            self.blur.data.out = self.pair2.target.clone();
            self.blur.data.direction = 1;
            self.blur.encode(encoder);
        }
        self.result.encode(encoder);
    }
}
