use std::time::SystemTime;

use winopengl::{
    Bindings, Buffer, BufferType, GraphicsContext, Pipeline, Shader, Texture,
    VertexAttribute, ShaderMeta, UniformBlockLayout, UniformDesc,
};

use crate::{pos::Pos, texture::TextureData, game::{SCREEN_WIDTH_FLOAT, SCREEN_HEIGHT_FLOAT}};

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

    pub fn rotate(&mut self, ctx: &mut GraphicsContext, rotation: ROTATION)
    {
        let square_vertices: [Vertex; 4] = get_rot_vertex(rotation, self.width, self.height);    
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &square_vertices);
        self.bindings.vertex_buffers = vec![vertex_buffer];
    }

    pub fn draw(&mut self, ctx: &mut GraphicsContext, pos:Pos) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&Uniforms {
            offset: (
                self.x_offset + (2. * f32::from(pos.x) / (SCREEN_WIDTH_FLOAT/self.width)) - 1.,
                self.y_offset + (2. * f32::from(pos.y) / (SCREEN_HEIGHT_FLOAT/self.height)) - 1.,
            )
        });
        ctx.draw(0, 6, 1);
    }

    pub(crate) fn new(ctx: &mut GraphicsContext, texture: TextureData, body: bool) -> GraphicalObject 
    { 
        let widhtf = f32::from(texture.width);
        let heightf = f32::from(texture.height);

        let x_offset = widhtf / SCREEN_WIDTH_FLOAT;
        let y_offset = heightf / SCREEN_HEIGHT_FLOAT;
        let now = SystemTime::now();

        GraphicalObject {
            width: widhtf,
            height: heightf,
            x_offset,
            y_offset,
            bindings: init_bindings(ctx, texture),
            pipeline: init_pipeline(ctx, body),
            time: now
        }
    }       
}

fn init_bindings(ctx: &mut GraphicsContext, texture: TextureData, ) -> Bindings {
    let widthf = f32::from(texture.width);
    let heightf = f32::from(texture.height);
    let square_vertices: [Vertex; 4] = get_rot_vertex(ROTATION::None, widthf, heightf);
    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &square_vertices);

    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

    let texture = Texture::new(ctx, &texture.data as &[u8], texture.width as _, texture.height as _);
    
    Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer,
        images: vec![texture],
    }
}

fn init_pipeline(ctx: &mut GraphicsContext, body: bool) -> Pipeline {    
    let vertex_shader:&str = std::str::from_utf8(include_bytes!("./shaders/shader.vs")).unwrap();
    let mut fragment_shader = std::str::from_utf8(include_bytes!("./shaders/shader.fs")).unwrap();
    if body
    {
        fragment_shader = std::str::from_utf8(include_bytes!("./shaders/body_shader.fs")).unwrap();
    }
        
    let shader = Shader::new(ctx, vertex_shader, fragment_shader, meta()).unwrap();

    Pipeline::new(
        ctx,
        &[
            VertexAttribute::new("pos"),
            VertexAttribute::new("uv"),
        ],
        shader)
}

fn get_rot_vertex(rotation: ROTATION, width: f32, height: f32) -> [Vertex; 4] {
    match rotation {
        ROTATION::None =>
        {
            [
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH_FLOAT, y: -height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH_FLOAT, y: -height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH_FLOAT, y:  height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH_FLOAT, y:  height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 0., y: 0. } },
            ]
        },
        ROTATION::Clockwise90 =>
        {
            [
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH_FLOAT, y: -height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH_FLOAT, y: -height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH_FLOAT, y:  height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH_FLOAT, y:  height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 1., y: 1. } },
            ]
        },
        ROTATION::Clockwise180 =>
        {
            [
                Vertex { pos: Vec2 { x: -width / SCREEN_WIDTH_FLOAT, y: -height / SCREEN_HEIGHT_FLOAT, }, uv: Vec2 { x: 0., y: 0. }, },
                Vertex { pos: Vec2 { x: width / SCREEN_WIDTH_FLOAT, y: -height / SCREEN_HEIGHT_FLOAT, }, uv: Vec2 { x: 1., y: 0. }, },
                Vertex { pos: Vec2 { x: width / SCREEN_WIDTH_FLOAT, y: height / SCREEN_HEIGHT_FLOAT, }, uv: Vec2 { x: 1., y: 1. }, },
                Vertex { pos: Vec2 { x: -width / SCREEN_WIDTH_FLOAT, y: height / SCREEN_HEIGHT_FLOAT, }, uv: Vec2 { x: 0., y: 1. }, },
            ]
        },
        ROTATION::Clockwise270 =>
        {
            [
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH_FLOAT, y: -height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH_FLOAT, y: -height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH_FLOAT, y:  height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH_FLOAT, y:  height/SCREEN_HEIGHT_FLOAT }, uv: Vec2 { x: 0., y: 0. } },
            ]
        }
    }
}



#[repr(C)]
pub struct Uniforms 
{
    pub offset: (f32, f32)
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
            uniforms: vec![UniformDesc::new("offset")],
        },
    }
}