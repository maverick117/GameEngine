#version 330

uniform vec3 eyePos;
uniform vec4 lightPos;
uniform vec3 lightColor;
uniform vec3 attenuation;
uniform float radius;

uniform sampler2D gPosition;
uniform sampler2D gNormal;

in vec2 frag_texcoord;

layout (location = 0) out vec4 light_output;

void main() {
  vec3 fragPos = texture(gPosition, frag_texcoord).xyz;
  vec3 fragNorm = texture(gNormal, frag_texcoord).xyz;
  vec3 fragToLight = lightPos.xyz - fragPos;
  vec3 fragToEye = eyePos.xyz - fragPos;
  float diffuse_coefficient = max(dot(normalize(fragNorm), normalize(fragToLight)),0.0);
  float specular_coefficient = dot(normalize(normalize(fragToEye) + normalize(fragToLight)), normalize(fragNorm));
  light_output.rgb = lightColor.rgb * diffuse_coefficient;
  light_output.a = specular_coefficient;
  //light_output = vec4(.5,.5,.5,.5);
}
