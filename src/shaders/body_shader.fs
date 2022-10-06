#version 100
varying lowp vec2 texcoord;
uniform lowp float time;
uniform sampler2D tex;

void main() {
    lowp float max = 0.9;
    lowp float min = 0.1;
    
    lowp vec4 res = texture2D(tex, texcoord);
    if (res.r > max)
    {
        discard;
    }
    else 
    {
        if (res.g > min)
        {
            if (res.g < 100./255.)
            {
                if (time <= 0.6)
                {  
                    res.g = 0.8;
                }
                else
                {
                    res.g = 0.4;
                }
            }
            else if (res.g < 200./255.)
            {
                if (time <= 0.8 && time > 0.2)
                {  
                    res.g = 0.8;
                }
                else
                {
                    res.g = 0.4;
                }
            }
            else 
            {
                if (time > 0.4)
                {   
                    res.g = 0.8;
                }
                else
                {
                    res.g = 0.4;
                }
            }
        } 
        else 
        {
            res.r = 0.;
            res.g = 0.;
            res.b = 0.;
        } 
        gl_FragColor = res;
    }
}