pub mod builder;
pub mod mesh;
pub mod shader;

use glow::{Context, HasContext};
use glutin::{
    context::PossiblyCurrentContext,
    surface::{GlSurface, Surface, WindowSurface},
};
use std::num::NonZeroU32;

pub struct Renderer {
    surface: Option<Surface<WindowSurface>>,
    context: Option<PossiblyCurrentContext>,
    gl: Option<Context>,
}

impl Renderer {
    pub fn get_gl(&self) -> &Context {
        self.gl.as_ref().unwrap()
    }

    pub fn clear(&self) {
        let gl = self.gl.as_ref().unwrap();

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.surface.as_ref().unwrap().resize(
            self.context.as_ref().unwrap(),
            NonZeroU32::new(width).unwrap(),
            NonZeroU32::new(height).unwrap(),
        );
    }

    pub fn swap_buffers(&self) {
        self.surface
            .as_ref()
            .unwrap()
            .swap_buffers(self.context.as_ref().unwrap())
            .unwrap();
    }
}
