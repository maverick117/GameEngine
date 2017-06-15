#version 330
uniform sampler2D albedo_texture;
uniform sampler2D specular_texture;
uniform sampler2D lighting_texture;

in vec2 frag_texcoord;

out vec4 frag_output;

void main(){
    frag_output = vec4(texture(albedo_texture, frag_texcoord).rgb , 1.0);






}
