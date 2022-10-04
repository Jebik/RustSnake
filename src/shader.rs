pub mod shader {
    use miniquad::*;

    pub const VERTEX: &str = r#"#version 100
    attribute vec2 pos;
    attribute vec2 uv;
    uniform vec2 offset;
    varying lowp vec2 texcoord;
    void main() {
        gl_Position = vec4(pos + offset, 0, 1);
        texcoord = uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 100
    varying lowp vec2 texcoord;
    uniform sampler2D tex;
    void main() {
        gl_FragColor = texture2D(tex, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["tex".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("offset", UniformType::Float2)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms 
    {
        pub offset: (f32, f32),
    }

    #[repr(C)]
    pub struct Vec2 {
        pub(crate) x: f32,
        pub(crate) y: f32,
    }
    #[repr(C)]
    pub struct Vertex {
        pub(crate) pos: Vec2,
        pub(crate) uv: Vec2,
    }
}