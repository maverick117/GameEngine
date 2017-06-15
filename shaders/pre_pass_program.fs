#version 330

//uniform sampler2D ambient_tex;
uniform sampler2D diffuse_tex;
//uniform sampler2D specular_tex;
//uniform sampler2D normal_tex;
//uniform sampler2D dissolve_tex;

in vec4 frag_pos;
in vec4 frag_norm;
in vec2 frag_texcoord;
in vec3 frag_diffuse;
in float frag_shininess;
in vec3 frag_specular;

layout (location = 0) out vec4 gPosition;
layout (location = 1) out vec4 gNormal;
layout (location = 2) out vec4 gAlbedo;
layout (location = 3) out vec4 gSpec;

void main() {
    gPosition = frag_pos;
    gNormal = frag_norm;
    gAlbedo.rgb = texture(diffuse_tex, frag_texcoord).rgb + frag_diffuse;
    gAlbedo.a = 1.0;
    gSpec = vec4(frag_specular, 1.0);
}
