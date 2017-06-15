#version 330

uniform vec4 eyePos;
uniform vec3 lightPos;
uniform vec3 lightColor;
uniform vec3 attenuation;
uniform float radius;

uniform sampler2D gPosition;
uniform sampler2D gNormal;

in vec2 frag_texcoord;

layout (location = 0) out vec4 frag_output;

void main() {
  vec3 fragPos = texture(gPosition, frag_texcoord).xyz;
  vec3 fragNorm = texture(gNormal, frag_texcoord).xyz;
  vec3 fragToLight = lightPos - fragPos;
  vec3 fragToEye = eyePos.xyz - fragPos;
  float diffuse_coefficient = clamp(dot(normalize(fragNorm), normalize(fragToLight)),0.0,1.0);
  float specular_coefficient = dot(normalize(normalize(fragToEye) + normalize(fragToLight)), normalize(fragNorm));
  //frag_output.rgb = lightColor ;//* diffuse_coefficient;

  //frag_output.a = specular_coefficient;
  frag_output.rgb = vec3(1.0,1.0,1.0);
  frag_output.a = 1.0;
}
