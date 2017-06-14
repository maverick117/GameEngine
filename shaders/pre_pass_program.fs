#version 330

uniform sampler2D tex;

in vec4 frag_pos;
in vec4 frag_norm;
in vec2 frag_texcoord;
in vec3 frag_diffuse;
in vec3 frag_specular;

layout (location = 0) out vec4 gPosition;
layout (location = 1) out vec4 gNormal;
layout (location = 2) out vec4 gAlbedo;
layout (location = 3) out vec4 gSpec;

void main() {
    gPosition = frag_pos;
    gNormal = frag_norm;
    gAlbedo = texture(tex, frag_texcoord) + vec4(frag_diffuse,1.0);
    gSpec = vec4(frag_specular,25.0);
}
