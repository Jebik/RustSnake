#version 100
varying lowp vec2 texcoord;
uniform lowp float time;
uniform sampler2D tex;

void main() {
    lowp float max = 0.9;
    lowp float min = 0.1;
    
    lowp vec4 res = texture2D(tex, texcoord);
    clip(max - res.r);
    
    lowp float external = 0.8-step(0.6, time)*0.4;
    lowp float middle = res.g = 0.8-step(0.8, time)*0.4+step(0.2, time)*0.4;
    lowp float center = res.g = step(0.4, time)*0.4 + 0.4;

    lowp float green = external-step(0.4. res.g)*external + middle-step(0.8. res.g)*middle + step(0.8 res.g)*center

    res.r = 0.;
    res.g = 0. + step(min, reg.g)*green;
    res.b = 0.;
    gl_FragColor = res;
}