#version 150 core

uniform sampler2D t_Buffer;
uniform int i_direction;
in vec2 v_TexCoord;
out vec4 Target0;

void main() {
    Target0 = vec4(1.0, 0.0, 0.0, 1.0);
}
