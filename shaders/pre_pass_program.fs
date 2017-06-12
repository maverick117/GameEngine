#version 330

uniform sampler2D tex;

in vec4 frag_pos;
in vec4 frag_norm;
in vec2 frag_texcoord;

out vec4 output1;
out vec4 output2;
out vec4 output3;
out vec4 output4;

void main() {
    output1 = frag_pos;
    output2 = frag_norm;
    output3 = texture(tex, frag_texcoord);
    output4 = vec4(1.0, 0.0, 1.0, 1.0);
}
