use std::num::NonZeroU32;

use glow::{Context, FRAGMENT_SHADER, HasContext, VERTEX_SHADER};
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
    pub fn new() -> Self {
        Self {
            surface: None,
            context: None,
            gl: None,
        }
    }
    pub fn create(&mut self, window_handle: WindowHandle<'_>, display_handle: DisplayHandle<'_>) {
        let preference = DisplayApiPreference::EglThenGlx(Box::new(|_| {}));
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

    pub fn draw(&self) {
        let gl = self.gl.as_ref().unwrap();

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }

        self.surface
            .as_ref()
            .unwrap()
            .swap_buffers(self.context.as_ref().unwrap())
            .unwrap();
    }

    pub fn load_shader(&self, vertex_shader: &str, fragment_shader: &str) {
        println!("load_shader");
        let gl = self.gl.as_ref().unwrap();

        unsafe {
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");
            gl.bind_vertex_array(Some(vertex_array));

            let program = gl.create_program().expect("Cannot create program");

            let shader_sources = [
                (VERTEX_SHADER, vertex_shader),
                (FRAGMENT_SHADER, fragment_shader),
            ];

            let mut shaders = Vec::with_capacity(shader_sources.len());

            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, &format!("{}\n{}", "#version 410", shader_source));
                gl.compile_shader(shader);

                if !gl.get_shader_compile_status(shader) {
                    panic!("{}", gl.get_shader_info_log(shader));
                }
                gl.attach_shader(program, shader);
                shaders.push(shader);
            }

            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                panic!("{}", gl.get_program_info_log(program));
            }

            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }

            gl.use_program(Some(program));
            gl.clear_color(0.0, 0.0, 0.0, 1.0);
        }
    }
}
