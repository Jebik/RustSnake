use std::time::{SystemTime, Duration};

use miniquad::{
    Bindings, Buffer, BufferLayout, BufferType, Context, Pipeline, Shader, Texture,
    VertexAttribute, VertexFormat, TextureParams, ShaderMeta, UniformBlockLayout, UniformDesc, UniformType,
};
use webp::Decoder;

const SCREEN_WIDTH: f32 = 1600.;
const SCREEN_HEIGHT: f32 = 896.;

pub enum ROTATION
{
    None,
    Clockwise90,
    Clockwise180,
    Clockwise270
}
pub struct GraphicalObject {
    //ForDrawing
    width: f32,
    height: f32,
    x_offset: f32,
    y_offset: f32,
    bindings: Bindings,
    pipeline: Pipeline,
    time: SystemTime
}
impl GraphicalObject {
    pub fn start_shader_time(&mut self)
    {
        self.time = SystemTime::now();
    }

    pub fn rotate(&mut self, ctx: &mut Context, rotation: ROTATION)
    {
        let square_vertices: [Vertex; 4] = get_rot_vertex(rotation, self.width, self.height);    
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &square_vertices);
        self.bindings.vertex_buffers = vec![vertex_buffer];
    }

    pub fn draw(&mut self, ctx: &mut Context, x: f32, y: f32) {
        let delta = self.time.elapsed().unwrap_or(Duration::from_secs(0)).as_secs_f32() % 2.;
        let ratio = delta/2.;
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&Uniforms {
            offset: (
                self.x_offset + (2. * x / (SCREEN_WIDTH/self.width)) - 1.,
                self.y_offset + (2. * y / (SCREEN_HEIGHT/self.height)) - 1.,
            ),
            time: ratio
        });
        ctx.draw(0, 6, 1);
    }

    pub(crate) fn new(ctx: &mut Context, file: &[u8], body: bool) -> GraphicalObject 
    { 
        let file = Decoder::new(file);
        let res = file.decode().unwrap();
        
        //LOADING IMAGE;
        let width:u16 = res.width() as _;
        let height:u16 = res.height() as _;
        let img = res.to_image();
        let texture = img.to_rgb8();
        let data = &texture as &[u8];

        let widhtf = f32::from(width);
        let heightf = f32::from(height);

        let x_offset = widhtf / SCREEN_WIDTH;
        let y_offset = heightf / SCREEN_HEIGHT;
        let now = SystemTime::now();

        GraphicalObject {
            width: widhtf,
            height: heightf,
            x_offset,
            y_offset,
            bindings: init_bindings(ctx, data, width, height),
            pipeline: init_pipeline(ctx, body),
            time: now
        }
    }       
}

fn init_bindings(ctx: &mut Context, data: &[u8], width: u16, height: u16) -> Bindings {
    let widthf = f32::from(width);
    let heightf = f32::from(height);
    let square_vertices: [Vertex; 4] = get_rot_vertex(ROTATION::None, widthf, heightf);
    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &square_vertices);

    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

    let param = TextureParams
    {
        format: miniquad::TextureFormat::RGB8,
        wrap: miniquad::TextureWrap::Clamp,
        filter: miniquad::FilterMode::Linear,
        width: width as _,
        height: height as _,
    };
    let texture = Texture::from_data_and_format(ctx, data, param);
    
    Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![texture],
    }
}

fn init_pipeline(ctx: &mut Context, body: bool) -> Pipeline {    
    let vertex_shader:&str = std::str::from_utf8(include_bytes!("./shaders/shader.vs")).unwrap();
    let mut fragment_shader = std::str::from_utf8(include_bytes!("./shaders/shader.fs")).unwrap();
    if body
    {
        fragment_shader = std::str::from_utf8(include_bytes!("./shaders/body_shader.fs")).unwrap();
    }
        
    let shader = Shader::new(ctx, vertex_shader, fragment_shader, meta()).unwrap();

    Pipeline::new(
        ctx,
        &[BufferLayout::default()],
        &[
            VertexAttribute::new("pos", VertexFormat::Float2),
            VertexAttribute::new("uv", VertexFormat::Float2),
        ],
        shader,)
}

fn get_rot_vertex(rotation: ROTATION, width: f32, height: f32) -> [Vertex; 4] {
    match rotation {
        ROTATION::None =>
        {
            [
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 0. } },
            ]
        },
        ROTATION::Clockwise90 =>
        {
            [
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 1. } },
            ]
        },
        ROTATION::Clockwise180 =>
        {
            [
                Vertex { pos: Vec2 { x: -width / SCREEN_WIDTH, y: -height / SCREEN_HEIGHT, }, uv: Vec2 { x: 0., y: 0. }, },
                Vertex { pos: Vec2 { x: width / SCREEN_WIDTH, y: -height / SCREEN_HEIGHT, }, uv: Vec2 { x: 1., y: 0. }, },
                Vertex { pos: Vec2 { x: width / SCREEN_WIDTH, y: height / SCREEN_HEIGHT, }, uv: Vec2 { x: 1., y: 1. }, },
                Vertex { pos: Vec2 { x: -width / SCREEN_WIDTH, y: height / SCREEN_HEIGHT, }, uv: Vec2 { x: 0., y: 1. }, },
            ]
        },
        ROTATION::Clockwise270 =>
        {
            [
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 0. } },
            ]
        }
    }
}



#[repr(C)]
pub struct Uniforms 
{
    pub offset: (f32, f32),
    pub time: f32
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

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        images: vec!["tex".to_string()],
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new("offset", UniformType::Float2), UniformDesc::new("time", UniformType::Float1)],
        },
    }
}