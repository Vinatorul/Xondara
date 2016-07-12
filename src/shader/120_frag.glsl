#version 120


uniform sampler2D t_Test;
varying vec2 v_Tc;

void main() {
    gl_FragColor = texture2D(t_Test, v_Tc);

}
