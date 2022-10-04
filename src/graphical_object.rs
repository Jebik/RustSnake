use crate::shader::shader::{meta, Uniforms, Vec2, Vertex, FRAGMENT, VERTEX};
use miniquad::{
    Bindings, Buffer, BufferLayout, BufferType, Context, Pipeline, Shader, Texture,
    VertexAttribute, VertexFormat,
};

const SCREEN_WIDTH: f32 = 1600.;
const SCREEN_HEIGHT: f32 = 896.;

pub enum ROTATION
{
    NONE,
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
}
impl GraphicalObject {
    pub fn rotate(&mut self, ctx: &mut Context, rotation: ROTATION)
    {
        let square_vertices: [Vertex; 4] = get_rot_vertex(rotation, self.width, self.height);    
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &square_vertices);
        self.bindings.vertex_buffers = vec![vertex_buffer];
    }

    pub fn draw(&mut self, ctx: &mut Context, x: f32, y: f32) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&Uniforms {
            offset: (
                self.x_offset + (2. * x / (SCREEN_WIDTH/self.width)) - 1.,
                self.y_offset + (2. * y / (SCREEN_HEIGHT/self.height)) - 1.,
            ),
        });
        ctx.draw(0, 6, 1);
    }

    pub(crate) fn new(ctx: &mut Context, texture: &[u8], width: i16, height: i16) -> GraphicalObject {
        let widhtf = f32::from(width);
        let heightf = f32::from(height);

        let x_offset = widhtf / SCREEN_WIDTH;
        let y_offset = heightf / SCREEN_HEIGHT;

        GraphicalObject {
            width: widhtf,
            height: heightf,
            x_offset,
            y_offset,
            bindings: init_bindings(ctx, texture, widhtf, heightf),
            pipeline: init_pipeline(ctx)
        }
    }       
}


fn init_bindings(ctx: &mut Context, texture: &[u8], width: f32, height: f32) -> Bindings {
    let square_vertices: [Vertex; 4] = get_rot_vertex(ROTATION::NONE, width, height);
    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &square_vertices);

    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

    let texture = Texture::from_rgba8(ctx, 64, 64, texture);

    Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer: index_buffer,
        images: vec![texture],
    }
}

fn init_pipeline(ctx: &mut Context) -> Pipeline {
    let shader = Shader::new(ctx, VERTEX, FRAGMENT, meta()).unwrap();

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
        ROTATION::NONE =>
        {
            [
                Vertex { pos: Vec2 { x: -width / SCREEN_WIDTH, y: -height / SCREEN_HEIGHT, }, uv: Vec2 { x: 0., y: 0. }, },
                Vertex { pos: Vec2 { x: width / SCREEN_WIDTH, y: -height / SCREEN_HEIGHT, }, uv: Vec2 { x: 1., y: 0. }, },
                Vertex { pos: Vec2 { x: width / SCREEN_WIDTH, y: height / SCREEN_HEIGHT, }, uv: Vec2 { x: 1., y: 1. }, },
                Vertex { pos: Vec2 { x: -width / SCREEN_WIDTH, y: height / SCREEN_HEIGHT, }, uv: Vec2 { x: 0., y: 1. }, },
            ]
        },
        ROTATION::Clockwise90 =>
        {
            [
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 0. } },
            ]
        },
        ROTATION::Clockwise180 =>
        {
            [
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 0. } },
            ]
        },
        ROTATION::Clockwise270 =>
        {
            [
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y: -height/SCREEN_HEIGHT }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x: -width/SCREEN_WIDTH, y:  height/SCREEN_HEIGHT }, uv: Vec2 { x: 1., y: 1. } },
            ]
        }
    }
}