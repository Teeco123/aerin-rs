use glow::{Context, HasContext};

#[derive(Clone)]
pub struct Mesh {
    vao: glow::VertexArray,
    vbo: glow::Buffer,
    ebo: glow::Buffer,
    index_count: i32,
}

impl Mesh {
    pub fn new(gl: &Context, vertices: &[f32], indices: &[u32]) -> Self {
        unsafe {
            let vao = gl.create_vertex_array().expect("Failed to create VAO");
            let vbo = gl.create_buffer().expect("Failed to create VBO");
            let ebo = gl.create_buffer().expect("Failed to create EBO");

            gl.bind_vertex_array(Some(vao));

            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            let vertices_bytes: &[u8] = core::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * core::mem::size_of::<f32>(),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_bytes, glow::STATIC_DRAW);

            gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
            let indices_bytes: &[u8] = core::slice::from_raw_parts(
                indices.as_ptr() as *const u8,
                indices.len() * core::mem::size_of::<u32>(),
            );
            gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, indices_bytes, glow::STATIC_DRAW);

            let stride = (6 * std::mem::size_of::<f32>()) as i32;
            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(0);

            gl.vertex_attrib_pointer_f32(1, 3, glow::FLOAT, false, stride, 12);
            gl.enable_vertex_attrib_array(1);

            gl.bind_vertex_array(None);

            Self {
                vao,
                vbo,
                ebo,
                index_count: indices.len() as i32,
            }
        }
    }

    pub fn draw(&self, gl: &Context) {
        unsafe {
            gl.bind_vertex_array(Some(self.vao));

            gl.draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_INT, 0);

            gl.bind_vertex_array(None);
        }
    }
}
