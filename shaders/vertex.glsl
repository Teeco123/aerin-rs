layout (location = 0) in vec3 a_position;
layout (location = 1) in vec3 a_color;

out vec3 v_color;

layout (std140) uniform Camera {
    mat4 u_view;
    mat4 u_projection;
};

uniform mat4 u_model;

void main() {
    gl_Position = u_projection * u_view * u_model * vec4(a_position, 1.0);
		v_color = a_color;
}
