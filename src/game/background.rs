use miniquad::{Bindings, Pipeline, Context, BufferLayout, VertexAttribute, VertexFormat, Buffer, Texture, Shader, BufferType};

use crate::shader::shader::{Vec2, Vertex, VERTEX, FRAGMENT, meta, Uniforms};

use crate::images::snake_bg1;
use crate::images::snake_bg2;
use crate::images::snake_bg3;
use crate::images::snake_bg4;

struct BackgroundItem
{
    pipeline: Pipeline,
    bindings: Bindings
}
impl BackgroundItem {    
    pub(crate) fn new(ctx: &mut Context, texture: &[u8], pos: u8) -> BackgroundItem 
    {
        BackgroundItem
        {
            pipeline: init_bg_pipeline(ctx),
            bindings: init_bg_bindings(ctx, texture, pos),
        }
    }
}

//OPENGL WEIRD
fn init_bg_pipeline(ctx: &mut Context)  -> Pipeline{
    let shader = Shader::new(ctx, VERTEX, FRAGMENT, meta()).unwrap();

    Pipeline::new(
        ctx,
        &[BufferLayout::default()],
        &[
            VertexAttribute::new("pos", VertexFormat::Float2),
            VertexAttribute::new("uv", VertexFormat::Float2),
        ],
        shader,
    )
}

fn init_bg_bindings(ctx: &mut Context, texture: &[u8], pos: u8)  -> Bindings {    
    let bg_vertices = get_vertices_from_pos(pos);
    
    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &bg_vertices);
    
    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

    let texture = Texture::from_rgba8(ctx, 800, 448, texture);

    Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer: index_buffer,
        images: vec![texture],
    }
}

fn get_vertices_from_pos(pos: u8) -> [Vertex; 4] {
    match pos{
        1 => 
            [
                Vertex { pos : Vec2 { x: -1., y: 0. }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  0., y: 0. }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  0., y:  1. }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x: -1., y:  1. }, uv: Vec2 { x: 0., y: 1. } },
            ],
        2 => 
            [
                Vertex { pos : Vec2 { x: 0., y: 0. }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  1., y: 0. }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  1., y:  1. }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x: 0., y:  1. }, uv: Vec2 { x: 0., y: 1. } },
            ],
        3 => 
            [
                Vertex { pos : Vec2 { x: -1., y: -1. }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  0., y: -1. }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  0., y:  0. }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x: -1., y:  0. }, uv: Vec2 { x: 0., y: 1. } },
            ],
        4 => 
            [
                Vertex { pos : Vec2 { x: 0., y: -1. }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  1., y: -1. }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  1., y:  0. }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x: 0., y:  0. }, uv: Vec2 { x: 0., y: 1. } },
            ],
            //Not USED
        _ =>
        [
            Vertex { pos : Vec2 { x: -1., y: -1. }, uv: Vec2 { x: 0., y: 0. } },
            Vertex { pos : Vec2 { x:  1., y: -1. }, uv: Vec2 { x: 1., y: 0. } },
            Vertex { pos : Vec2 { x:  1., y:  1. }, uv: Vec2 { x: 1., y: 1. } },
            Vertex { pos : Vec2 { x: -1., y:  1. }, uv: Vec2 { x: 0., y: 1. } },
        ]
    }
}


pub(crate) struct Background 
{
    bg_items: [BackgroundItem; 4]
}
impl Background {
    pub(crate) fn new(ctx: &mut Context) -> Background 
    {  
        Background
        {
            bg_items:
            [
                BackgroundItem::new(ctx, &snake_bg1::SNAKE_BG_RGB, 1),
                BackgroundItem::new(ctx, &snake_bg2::SNAKE_BG_RGB, 2),
                BackgroundItem::new(ctx, &snake_bg3::SNAKE_BG_RGB, 3),
                BackgroundItem::new(ctx, &snake_bg4::SNAKE_BG_RGB, 4)
            ]
        }
    }    
    
    pub fn draw(&mut self, ctx: &mut Context) 
    {
        for b in &self.bg_items
        {
            ctx.apply_pipeline(&b.pipeline);
            ctx.apply_bindings(&b.bindings);
            ctx.apply_uniforms(&Uniforms {
                offset: (0.,0.)
            });       
            ctx.draw(0, 6, 1);
        }         
    }
}