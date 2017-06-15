#version 330

uniform vec3 eyePos;

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
    frag_output.rgb = albedo_color.rgb * 0.2;
    frag_output.rgb += albedo_color.rgb * lighting_data.xyz;
    frag_output.rgb += specular_color.rgb * lighting_data.w;
    frag_output.a = 1.0;
  }
  else {
    // Generate skybox
    position_color = vec3(frag_texcoord.x * 40 - 20, frag_texcoord.y * 40 - 20, 0.0);
    vec3 eyeToFrag = normalize(position_color - eyePos);
    float absX = abs(eyeToFrag.x);
    float absY = abs(eyeToFrag.y);
    float absZ = abs(eyeToFrag.z);

    bool xPositive = eyeToFrag.x > 0 ? true : false;
    bool yPositive = eyeToFrag.y > 0 ? true : false;
    bool zPositive = eyeToFrag.z > 0 ? true : false;


    float uc, vc;
    float x = eyeToFrag.x;
    float y = eyeToFrag.y;
    float z = eyeToFrag.z;
    // Positive X
    if(xPositive && absX >= absY && absX >= absZ){
      uc = -z;
      vc = y;
      frag_output.rgb = texture(skybox_px_tex,vec2(uc,vc)).rgb;
    }
    // Negative X
    else if (!xPositive && absX >= absY && absX >= absZ){
      uc = z;
    vc = y;
    frag_output.rgb = texture(skybox_nx_tex,vec2(uc,vc)).rgb;
    }
    // Positive Y
    else if (yPositive && absY >= absX && absY >= absZ){
      uc = x;
          vc = -z;
          frag_output.rgb = texture(skybox_py_tex,vec2(uc,vc)).rgb;
    }
    // Negative Y
    else if (!yPositive && absY >= absX && absY >= absZ){
      uc = x;
          vc = z;
          frag_output.rgb = texture(skybox_ny_tex,vec2(uc,vc)).rgb;
    }
    // Positive Z
    else if (zPositive && absZ >= absX && absZ >= absY){
      uc = x;
          vc = y;
          frag_output.rgb = texture(skybox_pz_tex,vec2(uc,vc)).rgb;
    }
    // Negative Z
    else if (!zPositive && absZ >= absX && absZ >= absY){
      uc = -x;
          vc = y;
          frag_output.rgb = texture(skybox_nz_tex,vec2(uc,vc)).rgb;
    }

    //frag_output.rgb = eyeToFrag;
    //frag_output.rgb = texture(skybox_px_tex,frag_texcoord).rgb;
    frag_output.a = 1.0;
  }


  //frag_output = lighting_data;
  //frag_output = vec4(albedo_color.xyz,1.0);
  //frag_output.xyz = normal_color;


}
