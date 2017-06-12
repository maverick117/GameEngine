#version 330

uniform sampler2D position_texture;
uniform sampler2D normal_texture;
uniform vec3 light_position;
uniform vec3 light_color;
uniform vec3 light_attenuation;
uniform float light_radius;

in vec2 frag_texcoord;

out vec4 frag_output;

void main() {
    vec4 position = texture(position_texture, frag_texcoord);
    vec4 normal = texture(normal_texture, frag_texcoord);
    vec3 light_vector = light_position.xyz - position.xyz;
    float light_distance = abs(length(light_vector));
    vec3 normal_vector = normalize(normal.xyz);
    float diffuse = max(dot(normal_vector, light_vector),0.0);
    float attenuation_factor = 1.0 / (light_attenuation.x + light_attenuation.y * light_distance + light_attenuation.z * light_distance * light_distance);
    attenuation_factor *= (1.0 - pow((light_distance / light_radius),2.0));
    attenuation_factor = diffuse * max(attenuation_factor , 0.0);
    frag_output = vec4(light_color*diffuse,1.0);
}
