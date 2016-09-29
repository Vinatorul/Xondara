use gfx;
use gfx::format::{Srgba8, Rgba16F};
pub type ColorFormat = Srgba8;
pub type HDRFormat = Rgba16F;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        color: [f32; 3] = "a_Color",
    }

    vertex ResultVertex {
        pos: [f32; 2] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

    pipeline scene {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        out: gfx::RenderTarget<HDRFormat> = "Target0",
    }

    pipeline result {
        vbuf: gfx::VertexBuffer<ResultVertex> = (),
        tex: gfx::TextureSampler<[f32; 4]> = "t_BloomTex",
        out: gfx::RenderTarget<ColorFormat> = "Target0",
    }
}
