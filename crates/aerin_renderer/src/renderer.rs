use aerin_window::Window;
use glow::{Context, HasContext};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version},
    display::{Display, DisplayApiPreference},
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::num::NonZeroU32;
use std::{ffi::CString, sync::Arc};

pub struct Renderer {
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    gl: Arc<Context>,
}

impl Renderer {
    pub fn new(window: &Window) -> Result<Self, Box<dyn std::error::Error>> {
        let window_handle = window.window_handle()?.as_raw();
        let display_handle = window.display_handle()?.as_raw();
        let (width, height) = window.inner_size();

        let preference = DisplayApiPreference::Cgl;
        let gl_display = unsafe { Display::new(display_handle, preference)? };

        let template = ConfigTemplateBuilder::new();
        let gl_config = unsafe {
            gl_display
                .find_configs(template.build())?
                .next()
                .ok_or("Failed to find suitable GL config")?
        };

        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version::new(4, 1))))
            .build(Some(window_handle));

        let not_current = unsafe { gl_display.create_context(&gl_config, &context_attributes)? };

        let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window_handle,
            NonZeroU32::new(width.max(1)).unwrap(),
            NonZeroU32::new(height.max(1)).unwrap(),
        );

        let surface = unsafe { gl_display.create_window_surface(&gl_config, &attrs)? };

        let context = not_current.make_current(&surface)?;

        let gl = Arc::new(unsafe {
            glow::Context::from_loader_function(|s| {
                gl_display.get_proc_address(&CString::new(s).unwrap()) as *const _
            })
        });

        Ok(Self {
            surface,
            context,
            gl,
        })
    }

    pub fn gl(&self) -> Arc<Context> {
        self.gl.clone()
    }

    pub fn clear(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            self.gl.clear_color(r, g, b, a);
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        if let (Some(w), Some(h)) = (NonZeroU32::new(width), NonZeroU32::new(height)) {
            self.surface.resize(&self.context, w, h);
            unsafe {
                self.gl.viewport(0, 0, width as i32, height as i32);
            }
        }
    }

    pub fn swap_buffers(&self) {
        self.surface.swap_buffers(&self.context).unwrap();
    }
}
