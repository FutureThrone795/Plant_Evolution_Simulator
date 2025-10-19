#version 140

in vec3 position;
in vec3 color;
out vec3 vertex_color;

uniform mat4 perspective;
uniform mat4 view;

void main() {
    vertex_color = color;
    gl_Position = perspective * view * vec4(position, 1.0);
}