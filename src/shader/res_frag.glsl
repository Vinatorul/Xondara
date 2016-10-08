#version 150 core

uniform sampler2D t_SceneTex;
uniform sampler2D t_BloomTex;
in vec2 v_TexCoord;
out vec4 Target0;

void main() {
    vec4 tex = texture(t_SceneTex, v_TexCoord);
    tex += texture(t_BloomTex, v_TexCoord);
    Target0 = tex;
}
