#version 330

uniform sampler2D position_texture;
uniform sampler2D normal_texture;
uniform sampler2D albedo_texture;
uniform sampler2D specular_texture;
uniform sampler2D lighting_texture;

in vec2 frag_texcoord;

layout (location = 0) out vec4 frag_output;

void main(){

  vec3 position_color = texture(position_texture, frag_texcoord).rgb;
  vec3 normal_color = texture(normal_texture, frag_texcoord).rgb;
  vec3 albedo_color = texture(albedo_texture, frag_texcoord).rgb;
  vec3 specular_color = texture(specular_texture, frag_texcoord).rgb;
  vec4 lighting_data = texture(lighting_texture, frag_texcoord);
  frag_output = lighting_data;
  //frag_output = vec4(normal_color.xyz,1.0);



}
