#version 100
varying lowp vec2 texcoord;
uniform sampler2D tex;

void main() {
    lowp float max = 0.6;
    lowp float min = 0.4;

    lowp vec4 res = texture2D(tex, texcoord);
    if (res.r > max && res.g < min && res.b < min)
    {
        discard;
    }
    else
    {
        gl_FragColor = res;
    }
}