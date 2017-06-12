#version 330

uniform mat4 proj_matrix;
uniform mat4 view_matrix;
uniform mat4 model_matrix;

in vec3 position;
in vec3 normal;
in vec2 texcoord;

out vec4 frag_pos;
out vec4 frag_norm;
out vec2 frag_texcoord;

void main() {
    frag_pos = model_matrix * vec4(position, 1.0); // World coordinates of position
    frag_norm = model_matrix * vec4(normal, 1.0);  // Normal Vector in world coordinate
    frag_texcoord = texcoord;
    gl_Position = proj_matrix * view_matrix * model_matrix * vec4(position,1.0);
}
