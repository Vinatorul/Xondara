#version 120

attribute vec2 a_Pos;
attribute vec2 a_Tc;
varying vec2 v_Tc;

void main() {
    v_Tc = a_Tc;
    gl_Position = vec4(a_Pos, 0.0, 1.0);
}
