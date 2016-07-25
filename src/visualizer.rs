use std::io::Cursor;
use gfx;
use gfx::traits::FactoryExt;
use gfx::handle::ShaderResourceView;
use gfx::tex;
use image;

pub type ColorFormat = gfx::format::Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

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
               fragment_shader: &'static [u8],
               client_width: usize,
               client_height: usize) 
            -> Visualizer<R, F> {
        let pso = factory.create_pipeline_simple(
            verex_shader,
            fragment_shader,
            pipe::new()
        ).unwrap();
        let width = 200;
        let height = 200;
        let tile_size = 8;
        let tile_set_w = 32;
        let tile_set_h = 32;
        let width_ratio = 1.0/(client_width/tile_size) as f32;
        let height_ratio = 1.0/(client_height/tile_size) as f32; 
        println!("{}, {}", width_ratio, height_ratio);    

        let texture_data = &include_bytes!("../assets/CB-Temple-E.png")[..];
        let test_texture = load_texture(&mut factory, texture_data);
        let texture_pixel_w = 1.0/256.0;
        let texture_pixel_h = 1.0/280.0;
        println!("{}, {}", texture_pixel_w, texture_pixel_h);

        let coef = 2.0;
        let mut vertex_data = Vec::<Vertex>::new();
        let mut index_data = Vec::<u32>::new();
        for i in 0..width {
            for j in 0..height {
                let tile_index = 66;
                let tile_h = tile_index / tile_set_w;
                let tile_w = tile_index - tile_set_w*tile_h;
                vertex_data.push(
                    Vertex {
                        pos: [(width_ratio*i as f32)*coef - 1.0, (height_ratio*j as f32)*coef - 1.0],
                        tc: [texture_pixel_w*(tile_w*tile_size) as f32, texture_pixel_h*((tile_h+1)*tile_size) as f32]
                    });
                vertex_data.push(
                    Vertex {
                        pos: [(width_ratio*i as f32)*coef - 1.0, (height_ratio*(j+1) as f32)*coef - 1.0],
                        tc: [texture_pixel_w*(tile_w*tile_size) as f32, texture_pixel_h*(tile_h*tile_size) as f32]
                    });
                vertex_data.push(
                    Vertex {
                        pos: [(width_ratio*(i+1) as f32)*coef - 1.0, (height_ratio*j as f32)*coef - 1.0],
                        tc: [texture_pixel_w*((tile_w + 1)*tile_size) as f32, texture_pixel_h*((tile_h+1)*tile_size) as f32]
                    });             
                vertex_data.push(
                    Vertex {
                        pos: [(width_ratio*(i+1) as f32)*coef - 1.0, (height_ratio*(j+1) as f32)*coef - 1.0],
                        tc: [texture_pixel_w*((tile_w + 1)*tile_size) as f32, texture_pixel_h*(tile_h*tile_size) as f32]
                    });
                index_data.push((i*4+j*4*width));
                index_data.push((i*4+j*4*width) + 1);
                index_data.push((i*4+j*4*width) + 2);
                index_data.push((i*4+j*4*width) + 1);
                index_data.push((i*4+j*4*width) + 2);
                index_data.push((i*4+j*4*width) + 3);
            }
        }
        let (vertex_buffer, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, &index_data[..]);

        
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
