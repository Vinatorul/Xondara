#version 150 core

uniform sampler2D t_Buffer;
uniform int i_direction;
in vec2 v_TexCoord;
out vec4 Target0;

float weight[11] = float[](
    0.050561,
    0.050309,
    0.049561,
    0.048338,
    0.046677,
    0.044625,
    0.042239,
    0.039583,
    0.036725,
    0.033735,
    0.03068
 );

void main() {
    vec4 temp = texture(t_Buffer, v_TexCoord)*weight[0];
    for (int i=1; i<8; i++) {
        vec2 v_offset;
        if (i_direction == 0)
            v_offset = vec2(0.0, float(i))/768.0;
        else
            v_offset = vec2(float(i), 0.0)/1024.0;
        temp += texture(t_Buffer, (v_TexCoord + v_offset)) * weight[i];
        temp += texture(t_Buffer, (v_TexCoord - v_offset)) * weight[i];
    }

    Target0 = temp;
}
