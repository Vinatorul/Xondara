use common;
use pipeline::Vertex;
use gfx;
use gfx::handle::ShaderResourceView;

pub struct Level<R> where R: gfx::Resources {
    pub texture: ShaderResourceView<R, [f32; 4]>,
    level_info: Vec<i32>,
    texture_pixel_w: f32,
    texture_pixel_h: f32,
}

impl<R> Level<R> where R: gfx::Resources {
    pub fn new<F>(factory: &mut F) -> Level<R>
        where F: gfx::Factory<R>
    {
        let texture_data = &include_bytes!("../assets/CB-Temple-E.png")[..];
        let (texture, texture_w, texture_h) = common::load_texture(factory, texture_data);
        let level_info = Vec::<i32>::new();

        Level {
            texture: texture,
            level_info: level_info,
            texture_pixel_w: 1.0/texture_w,
            texture_pixel_h: 1.0/texture_h,
        }
    }

    pub fn generate_mesh(&self, client_width: usize, client_height: usize) -> (Vec<Vertex>, Vec<u32>) {
        let width = 200;
        let height = 200;
        let tile_size = 8;
        let tile_set_w = 32;
        let tile_set_h = 32;
        let width_ratio = 1.0/(client_width/tile_size) as f32;
        let height_ratio = 1.0/(client_height/tile_size) as f32;

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
                        tc: [self.texture_pixel_w*(tile_w*tile_size) as f32, self.texture_pixel_h*((tile_h+1)*tile_size) as f32]
                    });
                vertex_data.push(
                    Vertex {
                        pos: [(width_ratio*i as f32)*coef - 1.0, (height_ratio*(j+1) as f32)*coef - 1.0],
                        tc: [self.texture_pixel_w*(tile_w*tile_size) as f32, self.texture_pixel_h*(tile_h*tile_size) as f32]
                    });
                vertex_data.push(
                    Vertex {
                        pos: [(width_ratio*(i+1) as f32)*coef - 1.0, (height_ratio*j as f32)*coef - 1.0],
                        tc: [self.texture_pixel_w*((tile_w + 1)*tile_size) as f32, self.texture_pixel_h*((tile_h+1)*tile_size) as f32]
                    });             
                vertex_data.push(
                    Vertex {
                        pos: [(width_ratio*(i+1) as f32)*coef - 1.0, (height_ratio*(j+1) as f32)*coef - 1.0],
                        tc: [self.texture_pixel_w*((tile_w + 1)*tile_size) as f32, self.texture_pixel_h*(tile_h*tile_size) as f32]
                    });
                index_data.push((i*4+j*4*width));
                index_data.push((i*4+j*4*width) + 1);
                index_data.push((i*4+j*4*width) + 2);
                index_data.push((i*4+j*4*width) + 1);
                index_data.push((i*4+j*4*width) + 2);
                index_data.push((i*4+j*4*width) + 3);
            }
        }
        (vertex_data, index_data)
    }
}
