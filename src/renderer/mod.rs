pub mod shader;

use std::{collections::HashMap, num::NonZeroU32};

use glow::{Context, HasContext, Program};
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
    programs: HashMap<String, Program>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            surface: None,
            context: None,
            gl: None,
            programs: HashMap::new(),
        }
    }
    pub fn init(&mut self, window_handle: WindowHandle<'_>, display_handle: DisplayHandle<'_>) {
        let preference = DisplayApiPreference::Cgl;
        let gl_display = unsafe { Display::new(display_handle.as_raw(), preference).unwrap() };

        let template = ConfigTemplateBuilder::new();

        let gl_config = unsafe {
            gl_display
                .find_configs(template.build())
                .unwrap()
                .next()
                .unwrap()
        };

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(3, 3))))
            .build(Some(window_handle.as_raw()));

        let not_current = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
        };

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle.as_raw(),
            NonZeroU32::new(800).unwrap(),
            NonZeroU32::new(800).unwrap(),
        );

        let surface = unsafe {
            gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap()
        };

        let context = not_current.make_current(&surface).unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                gl_display.get_proc_address(&std::ffi::CString::new(s).unwrap()) as *const _
            })
        };

        self.surface = Some(surface);
        self.context = Some(context);
        self.gl = Some(gl);
    }

    pub fn get_gl(&self) -> &Context {
        self.gl.as_ref().unwrap()
    }

    pub fn draw(&self) {
        let gl = self.gl.as_ref().unwrap();

        unsafe {
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
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
