#define resolution vec2(1024.0, 768.0)
#define Thickness 0.005

float drawLine(vec2 p1, vec2 p2) {
    vec2 uv = gl_FragCoord.xy / resolution.xy;

    float a = abs(distance(p1, uv));
    float b = abs(distance(p2, uv));
    float c = abs(distance(p1, p2));

    if ((a >= c) || (b >= c)) 
        return 0.0;

    float p = (a+b+c) * 0.5;

    float h = 2.0/c * sqrt(p*(p - a)*(p - b)*( p - c));
    if (h < Thickness) {
        float coef = 1.0/ (Thickness) * h;
        return mix(1.0, 0.05, coef);
    }
    else if (h < 4.0*Thickness) {
        float coef = 1.0/ (4.0*Thickness) * h;
        return mix(0.05, 0.0, coef);
    }
    else
        return 0.0;
}

void main() {
    gl_FragColor = vec4(1.0, 0.0, 0.0, 1.0) * vec4(
        max(
            max(
                max(drawLine(vec2(0.4, 0.6), vec2(0.6, 0.5)),
                    drawLine(vec2(0.4, 0.4), vec2(0.6, 0.5))),
                drawLine(vec2(0.4, 0.6), vec2(0.43, 0.5))),
            drawLine(vec2(0.4, 0.4), vec2(0.43, 0.5)))
    );
}
