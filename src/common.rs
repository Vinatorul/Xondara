use gfx;
use gfx::handle::ShaderResourceView;
use gfx::tex;
use std::io::Cursor;
use image;
use pipeline::ColorFormat;

pub fn _load_texture<R, F>(factory: &mut F,
                           data: &[u8])
                           -> (ShaderResourceView<R, [f32; 4]>, f32, f32)
    where R: gfx::Resources,
          F: gfx::Factory<R>
{
    let img = image::load(Cursor::new(data), image::PNG).unwrap().to_rgba();
    let (width, height) = img.dimensions();
    let kind = tex::Kind::D2(width as tex::Size, height as tex::Size, tex::AaMode::Single);
    let (_, view) = factory.create_texture_const_u8::<ColorFormat>(kind, &[&img]).unwrap();
    (view, width as f32, height as f32)
}

#[inline]
pub fn vertex_shader() -> &'static [u8] {
    include_bytes!("shader/vert.glsl")
}

#[inline]
pub fn fragment_shader() -> &'static [u8] {
    include_bytes!("shader/frag.glsl")
}

#[inline]
pub fn pp_vertex_shader() -> &'static [u8] {
    include_bytes!("shader/pp_vert.glsl")
}

#[inline]
pub fn blur_fragment_shader() -> &'static [u8] {
    include_bytes!("shader/blur_frag.glsl")
}

#[inline]
pub fn result_fragment_shader() -> &'static [u8] {
    include_bytes!("shader/res_frag.glsl")
}
