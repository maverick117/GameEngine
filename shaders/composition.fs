#version 330
uniform sampler2D albedo_texture;
uniform sampler2D specular_texture;
uniform sampler2D lighting_texture;

in vec2 frag_texcoord;

layout (location = 0) out vec4 frag_output;

void main(){


  vec3 albedo_color = texture(albedo_texture, frag_texcoord).rgb;
  vec3 specular_color = texture(specular_texture, frag_texcoord).rgb;
  vec4 lighting_data = texture(lighting_texture, frag_texcoord);
  frag_output = vec4(albedo_color + specular_color, 1.0);
  //frag_output = lighting_color;




}
