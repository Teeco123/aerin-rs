use std::num::NonZeroU32;

use crate::renderer::Renderer;

use winit::raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, Version},
    display::{Display, DisplayApiPreference},
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{SurfaceAttributesBuilder, WindowSurface},
};

pub struct RendererBuilder {
    width: u32,
    height: u32,
    window_handle: Option<RawWindowHandle>,
    display_handle: Option<RawDisplayHandle>,
}

impl RendererBuilder {
    pub fn new() -> Self {
        Self {
            width: 1280,
            height: 720,
            window_handle: None,
            display_handle: None,
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_window_handle(mut self, window_handle: RawWindowHandle) -> Self {
        self.window_handle = Some(window_handle);
        self
    }

    pub fn with_display_handle(mut self, display_handle: RawDisplayHandle) -> Self {
        self.display_handle = Some(display_handle);
        self
    }

    pub fn build(self) -> Result<Renderer, Box<dyn std::error::Error>> {
        let window_handle = self.window_handle.ok_or("Window handle is required")?;
        let display_handle = self.display_handle.ok_or("Display handle is required")?;

        let preference = DisplayApiPreference::Cgl;
        let gl_display = unsafe { Display::new(display_handle, preference).unwrap() };

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
            .build(Some(window_handle));

        let not_current = unsafe {
            gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap()
        };

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle,
            NonZeroU32::new(self.width).unwrap(),
            NonZeroU32::new(self.height).unwrap(),
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

        return Ok(Renderer {
            surface: Some(surface),
            context: Some(context),
            gl: Some(gl),
        });
    }
}
