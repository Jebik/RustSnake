use miniquad::{Pipeline, Bindings, Context, Shader, BufferLayout, VertexFormat, VertexAttribute, Buffer, BufferType, Texture, BlendState, Equation, BlendFactor, BlendValue};

use crate::{images::snake_bonus::SNAKE_BONUS_RGB};
use crate::shader::{shader::{Vertex, Vec2, Uniforms, VERTEX, FRAGMENT, meta}};

pub struct Bonus{    
    pub x:i16,
    pub y:i16,
    //ForDrawing
    pipeline: Pipeline,
    bindings: Bindings,
}
impl Bonus {
    pub(crate) fn new(ctx: &mut Context) -> Bonus {
        Bonus {
            x: 0,
            y: 0,
            pipeline: init_bonus_pipeline(ctx),
            bindings: init_bonus_bindings(ctx)
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) {
        let x_offset = 32./800.;
        let y_offset = 32./448.;
    
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&Uniforms {
            offset: (x_offset+(2.*f32::from(self.x)/25.) - 1. ,y_offset+(2.*f32::from(self.y)/14.) - 1.),
        });       
        ctx.draw(0, 6, 1);
    }    
}

fn init_bonus_pipeline(ctx: &mut Context)  -> Pipeline{
    let shader = Shader::new(ctx, VERTEX, FRAGMENT, meta()).unwrap();

    let p = Pipeline::new(
        ctx,
        &[BufferLayout::default()],
        &[
            VertexAttribute::new("pos", VertexFormat::Float2),
            VertexAttribute::new("uv", VertexFormat::Float2),
        ],
        shader,
    );
    
    p.set_blend(ctx, Some(BlendState::new(Equation::Add,
        BlendFactor::Value(BlendValue::SourceAlpha),
        BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),);
    p
}

fn init_bonus_bindings(ctx: &mut Context)  -> Bindings {   

    let square_vertices: [Vertex; 4] = [
        Vertex { pos : Vec2 { x: -32./800., y: -32./448. }, uv: Vec2 { x: 0., y: 0. } },
        Vertex { pos : Vec2 { x:  32./800., y: -32./448. }, uv: Vec2 { x: 1., y: 0. } },
        Vertex { pos : Vec2 { x:  32./800., y:  32./448.  }, uv: Vec2 { x: 1., y: 1. } },
        Vertex { pos : Vec2 { x: -32./800., y:  32./448.  }, uv: Vec2 { x: 0., y: 1. } },
    ];
    
    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &square_vertices);
    
    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

    let texture = Texture::from_rgba8(ctx, 64, 64, &SNAKE_BONUS_RGB);

    Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer: index_buffer,
        images: vec![texture],
    }
}