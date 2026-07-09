use std::{collections::HashMap, num::NonZeroU32};

use glow::{Context, FRAGMENT_SHADER, HasContext, Program, VERTEX_SHADER};
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version},
    display::{Display, DisplayApiPreference},
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use winit::raw_window_handle::{DisplayHandle, WindowHandle};

struct VertexShader {
    r#type: u32,
    source: String,
}

struct FragmentShader {
    r#type: u32,
    source: String,
}

pub struct ShaderSource {
    vertex_shader: VertexShader,
    fragment_shader: FragmentShader,
}

pub struct Renderer {
    surface: Option<Surface<WindowSurface>>,
    context: Option<PossiblyCurrentContext>,
    gl: Option<Context>,
    shaders_sources: HashMap<String, ShaderSource>,
    programs: HashMap<String, Program>,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            surface: None,
            context: None,
            gl: None,
            shaders_sources: HashMap::new(),
            programs: HashMap::new(),
        }
    }
    pub fn create(&mut self, window_handle: WindowHandle<'_>, display_handle: DisplayHandle<'_>) {
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

    pub fn draw(&self) {
        let gl = self.gl.as_ref().unwrap();

        self.use_shader("triangle".to_string());

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

    pub fn load_shader(&mut self, name: String, vertex_shader: String, fragment_shader: String) {
        let shader = ShaderSource {
            vertex_shader: VertexShader {
                r#type: VERTEX_SHADER,
                source: vertex_shader,
            },
            fragment_shader: FragmentShader {
                r#type: FRAGMENT_SHADER,
                source: fragment_shader,
            },
        };
        self.shaders_sources.insert(name, shader);
    }

    pub fn compile_shaders(&mut self) {
        let gl = self.gl.as_ref().unwrap();

        unsafe {
            for (name, shader) in self.shaders_sources.iter() {
                let vertex_array = gl
                    .create_vertex_array()
                    .expect("Cannot create vertex array");
                gl.bind_vertex_array(Some(vertex_array));

                let program = gl.create_program().expect("Cannot create program");
                let mut shaders = Vec::with_capacity(self.shaders_sources.len());

                let vertex_shader = gl
                    .create_shader(shader.vertex_shader.r#type)
                    .expect("Cannot create vertex shader");
                gl.shader_source(
                    vertex_shader,
                    &format!("{}\n{}", "#version 410", shader.vertex_shader.source),
                );

                let fragment_shader = gl
                    .create_shader(shader.fragment_shader.r#type)
                    .expect("Cannot create fragment shader");
                gl.shader_source(
                    fragment_shader,
                    &format!("{}\n{}", "#version 410", shader.fragment_shader.source),
                );

                gl.compile_shader(vertex_shader);
                if !gl.get_shader_compile_status(vertex_shader) {
                    panic!("{}", gl.get_shader_compile_status(vertex_shader))
                }

                gl.compile_shader(fragment_shader);
                if !gl.get_shader_compile_status(fragment_shader) {
                    panic!("{}", gl.get_shader_compile_status(fragment_shader))
                }

                gl.attach_shader(program, vertex_shader);
                gl.attach_shader(program, fragment_shader);

                shaders.push(vertex_shader);
                shaders.push(fragment_shader);

                gl.link_program(program);

                if !gl.get_program_link_status(program) {
                    panic!("{}", gl.get_program_info_log(program));
                }

                for shader in shaders {
                    gl.detach_shader(program, shader);
                    gl.delete_shader(shader);
                }

                self.programs.insert(name.to_string(), program);
            }
        }
    }

    pub fn use_shader(&self, name: String) {
        let gl = self.gl.as_ref().unwrap();
        let program = self
            .programs
            .get(&name)
            .unwrap_or_else(|| panic!("Program '{}' was not found!", name));
        unsafe {
            gl.use_program(Some(*program));
        }
    }
}
