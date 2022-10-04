use miniquad::{Context, Pipeline, Bindings, VertexAttribute, VertexFormat, Buffer, BufferType, Texture, BufferLayout, Shader, BlendState, Equation, BlendValue, BlendFactor};

use crate::{shader::shader::{Vec2, Vertex, VERTEX, FRAGMENT, meta, Uniforms}, images::snake_body::SNAKE_BODY_RGB};
pub struct SnakeBody{ 
    //ForDrawing
    pub real_x:f32,
    pub real_y:f32,
    //ForLogic
    pub curr_x:i16,
    pub curr_y:i16,
    pub dest_x:i16,
    pub dest_y:i16,
    //ForDrawing
    pipeline: Pipeline,
    bindings: Bindings,
}

impl SnakeBody {    
    pub(crate) fn new(ctx: &mut Context, x:i16, y:i16) -> SnakeBody {
        SnakeBody {
            real_x: f32::from(x),
            real_y: f32::from(y),
            curr_x: x,
            curr_y: y,
            dest_x: -1,
            dest_y: -1,
            pipeline: init_bonus_pipeline(ctx),
            bindings: init_bonus_bindings(ctx)
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.dest_x == -1 ||self.dest_y == -1
        {
            return;
        }
        let x_offset = 32./800.;
        let y_offset = 32./448.;
    
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&Uniforms {
            offset: (x_offset+(2.*self.real_x/25.) - 1. ,y_offset+(2.*self.real_y/14.) - 1.),
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

    let texture = Texture::from_rgba8(ctx, 64, 64, &SNAKE_BODY_RGB);

    Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer: index_buffer,
        images: vec![texture],
    }
}