#version 330
uniform mat4 matrix;

in vec4 position;
in vec2 texcoord;

out vec2 frag_texcoord;

void main(){
    gl_Position = matrix * position;
    frag_texcoord = texcoord;
}
