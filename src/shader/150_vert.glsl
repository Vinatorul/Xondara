#version 150 core

in vec2 a_Pos;
in vec2 a_Tc;
out vec2 v_Tc;

void main() {
    v_Tc = a_Tc;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
