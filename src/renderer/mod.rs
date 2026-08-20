pub mod builder;
pub mod mesh;
pub mod shader;

use std::num::NonZeroU32;

use glow::{Context, HasContext};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version},
    display::{Display, DisplayApiPreference},
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use winit::raw_window_handle::{DisplayHandle, WindowHandle};

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

    pub fn swap_buffers(&self) {
        self.surface
            .as_ref()
            .unwrap()
            .swap_buffers(self.context.as_ref().unwrap())
            .unwrap();
    }
}
