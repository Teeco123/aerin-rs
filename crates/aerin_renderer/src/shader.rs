use glow::{Context, FRAGMENT_SHADER, HasContext, NativeBuffer, NativeProgram, VERTEX_SHADER};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProgramId(pub(crate) NativeProgram);

#[derive(Clone)]
pub struct Shader {
    id: ProgramId,
}

impl Shader {
    pub fn new(gl: &Context, vertex_source: &str, fragment_source: &str) -> Self {
        unsafe {
            let program = gl.create_program().expect("Cannot create shader program");

            let vertex_shader = gl
                .create_shader(VERTEX_SHADER)
                .expect("Cannot create vertex shader");
            gl.shader_source(vertex_shader, &format!("#version 410\n{vertex_source}"));

            let fragment_shader = gl
                .create_shader(FRAGMENT_SHADER)
                .expect("Cannot create fragment shader");
            gl.shader_source(fragment_shader, &format!("#version 410\n{fragment_source}"));

            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                panic!(
                    "Vertex Shader Error: {}",
                    gl.get_shader_info_log(vertex_shader)
                );
            }

            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                panic!(
                    "Fragment Shader Error: {}",
                    gl.get_shader_info_log(fragment_shader)
                );
            }

            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                panic!("Shader Link Error: {}", gl.get_program_info_log(program));
            }

            gl.detach_shader(program, vertex_shader);
            gl.delete_shader(vertex_shader);
            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(fragment_shader);

            Self {
                id: ProgramId(program),
            }
        }
    }

    pub fn bind(&self, gl: &Context) {
        unsafe {
            gl.use_program(Some(self.id.0));
        }
    }

    pub fn set_uniform_mat4(&self, gl: &Context, name: &str, val: &[f32]) {
        unsafe {
            let location = gl.get_uniform_location(self.id.0, name);
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, val);
        }
    }

    pub fn set_uniform_f32(&self, gl: &Context, name: &str, val: f32) {
        unsafe {
            let location = gl.get_uniform_location(self.id.0, name);
            gl.uniform_1_f32(location.as_ref(), val);
        }
    }

    pub fn create_ubo(gl: &Context, index: u32, size: i32) -> NativeBuffer {
        unsafe {
            let ubo = gl.create_buffer().expect("Failed to create UBO");
            gl.bind_buffer(glow::UNIFORM_BUFFER, Some(ubo));
            gl.buffer_data_size(glow::UNIFORM_BUFFER, size, glow::DYNAMIC_DRAW);
            gl.bind_buffer_base(glow::UNIFORM_BUFFER, index, Some(ubo));
            gl.bind_buffer(glow::UNIFORM_BUFFER, None);
            ubo
        }
    }
}
