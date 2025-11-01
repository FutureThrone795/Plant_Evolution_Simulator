#version 140

in vec3 position;
in vec4 color;
out vec4 vertex_color;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform bool is_plant;
uniform float total_time;

const float FOG_MIN = 700.0;
const float FOG_MAX = 1024.0;

void main() {
    float x_offset = 0.0;
    float z_offset = 0.0;

    if (is_plant) {
        x_offset = 0.031 * pow(position.y, 1.2) * sin(0.5 * (total_time + cos(0.12 * position.x)));
        z_offset = 0.083 * pow(position.y, 1.2) * sin(0.7 * (total_time + cos(0.12 * position.z)));
    }

    vec4 pos = model * vec4(position.x + x_offset, position.y, position.z + z_offset, 1.0);

    vec4 worldspace = view * pos;
    float dist = length(worldspace);
    gl_Position = perspective * worldspace;

    vec3 ground_color = vec3(color.r, color.g, color.b) * min((position.y * 0.01 + 0.5), 1.1);
    vec3 fog_color = vec3(0.79, 0.88, 0.97);

    float fog_density = 0.0;
    if (dist >= FOG_MAX) {
        fog_density = 1.0;
    } else if (dist >= FOG_MIN) {
        fog_density = (dist - FOG_MIN) * (1.0 / (FOG_MAX - FOG_MIN));
    } 

    vertex_color = vec4(ground_color * (1.0 - fog_density) + fog_color * fog_density, color.a);
}