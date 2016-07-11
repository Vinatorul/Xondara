#version 150 core

uniform sampler2D t_Test;
in vec2 v_Tc;
out vec4 Target0;

void main() {
    Target0 = texture2D(t_Test, v_Tc);
}
