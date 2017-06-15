#version 330

uniform sampler2D position_texture;
uniform sampler2D normal_texture;
uniform sampler2D albedo_texture;
uniform sampler2D specular_texture;
uniform sampler2D lighting_texture;

uniform sampler2D skybox_px_tex;
uniform sampler2D skybox_py_tex;
uniform sampler2D skybox_pz_tex;
uniform sampler2D skybox_nx_tex;
uniform sampler2D skybox_ny_tex;
uniform sampler2D skybox_nz_tex;

in vec2 frag_texcoord;

layout (location = 0) out vec4 frag_output;

void main(){

  vec3 position_color = texture(position_texture, frag_texcoord).rgb;
  vec3 normal_color = texture(normal_texture, frag_texcoord).rgb;
  vec4 albedo = texture(albedo_texture, frag_texcoord);
  vec3 albedo_color = albedo.rgb;
  vec3 specular_color = texture(specular_texture, frag_texcoord).rgb;
  vec4 lighting_data = texture(lighting_texture, frag_texcoord);
  if(albedo.a == 1.0){
    frag_output.rgb = albedo_color.rgb * 0.5;
    frag_output.rgb += albedo_color.rgb * lighting_data.xyz;
    frag_output.rgb += specular_color.rgb * lighting_data.w;
    frag_output.a = 1.0;
  }
  else {
    // Generate skybox
    frag_output.rgb = texture(skybox_px_tex,frag_texcoord).rgb;
    frag_output.a = 1.0;
  }

  //frag_output = lighting_data;
  //frag_output = vec4(albedo_color.xyz,1.0);
  //frag_output.xyz = normal_color;


}
