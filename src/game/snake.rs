use std::time::{SystemTime, Duration};
use miniquad::{Context, Bindings, Buffer, BufferType, Texture, Shader, Pipeline, BufferLayout, VertexAttribute, VertexFormat};
use crate::{shader::shader::{Vertex, Vec2, self}, images::snake_head::SNAKE_HEAD_RGB};
use super::snake_body::SnakeBody;

#[derive(PartialEq, Clone, Copy)]
pub enum Dir
{
    LEFT,
    RIGHT,
    UP,
    DOWN
}
/*
25x14 BOARD
*/
pub struct Snake
{   
    //ForDrawing
    real_x:f32,
    real_y:f32,
    //ForLogic
    pub curr_x:i16,
    pub curr_y:i16,
    pub dest_x:i16,
    pub dest_y:i16,
    //rest
    pub dir: Dir,
    next_dir: Vec<Dir>,
    body_part:Vec<SnakeBody>,
    last_move_start: SystemTime,
    pipeline: Pipeline,
    bindings: Bindings
}
impl Snake {
    pub(crate) fn new(ctx: &mut Context) -> Snake 
    {
        Snake
        {
            next_dir: Vec::new(),
            body_part: Vec::new(),
            real_x: 12.,
            real_y: 7.,
            curr_x:12,
            curr_y:7,
            dest_x:13,
            dest_y:7,
            dir: Dir::RIGHT,            
            last_move_start:SystemTime::now(),
            //ForDrawing
            pipeline: init_head_pipeline(ctx),
            bindings: init_head_bindings(ctx, Dir::RIGHT)
        }
    }

    pub(crate) fn reset(&mut self) -> () {
        self.body_part = Vec::new();
        self.next_dir = Vec::new();
        self.real_x = 12.;
        self.real_y = 7.;
        self.curr_x = 12;
        self.curr_y = 7;
        self.dest_x = 13;
        self.dest_y = 7;
        self.dir = Dir::RIGHT;
    }    


    pub fn start(&mut self) -> () {
        self. last_move_start = SystemTime::now();
    }

    pub fn check_reach(&mut self)  -> bool
    {        
        let mut reach = false;
        let now = SystemTime::now();
        let move_duration = now.duration_since(self.last_move_start).unwrap();

        //TargetMoveRatio = Speed = Time to reach a case.
        let move_ratio = Duration::from_millis(250);

        if move_duration > move_ratio
        {   
            //we reach BIG EVENT
            reach = true;
            self.compute_target();
            self.last_move_start = SystemTime::now();    
        }
        else
        {
            let ratio = move_duration.as_secs_f32() / move_ratio.as_secs_f32();
            self.real_x = f32::from(&self.dest_x-&self.curr_x)*ratio + f32::from(self.curr_x);
            self.real_y = f32::from(&self.dest_y-&self.curr_y)*ratio + f32::from(self.curr_y);
            
            for b in self.body_part.iter_mut()
            {
                if b.dest_x != -1 && b.dest_y != -1
                {
                    b.real_x = f32::from(&b.dest_x-&b.curr_x)*ratio + f32::from(b.curr_x);
                    b.real_y = f32::from(&b.dest_y-&b.curr_y)*ratio + f32::from(b.curr_y);
                }
            }
        }

        //return true if reach dest
        reach
    }

    pub(crate) fn try_add(&mut self, dir: Dir) {
        if self.next_dir.len() > 1
        {
            return;
        }
        
        let last_dir = self.dir;

        match last_dir {
            Dir::LEFT => 
            {   
                if dir == Dir::UP || dir == Dir::DOWN
                {
                    self.next_dir.push(dir);
                }
            },
            Dir::RIGHT => 
            {
                if dir == Dir::UP || dir == Dir::DOWN
                {
                    self.next_dir.push(dir);
                }                
            },
            Dir::UP => 
            {
                if dir == Dir::LEFT || dir == Dir::RIGHT
                {
                    self.next_dir.push(dir);
                }                
            },
            Dir::DOWN => 
            {
                if dir == Dir::LEFT || dir == Dir::RIGHT
                {
                    self.next_dir.push(dir);
                }                
            }
        }
        
    }

    
    pub fn grow(&mut self, ctx: &mut Context) 
    {
        let mut x = self.curr_x;
        let mut y = self.curr_y;

        let last_body = self.body_part.last();
        if last_body.is_some()
        {
            let body = last_body.unwrap();
            x = body.curr_x;
            y = body.curr_y;
        }

        self.body_part.push(SnakeBody::new(ctx, x, y))
    }

    
    pub fn draw(&mut self, ctx: &mut Context) {
        let x_offset = 32./800.;
        let y_offset = 32./448.;
    
        /*let t = date::now();
        eprintln!("{:#?}", t);*/
        
        let square_vertices: [Vertex; 4] = get_dir_vertex(&self.dir);    
        let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &square_vertices);
        self.bindings.vertex_buffers = vec![vertex_buffer];

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&shader::Uniforms {
            offset: (x_offset+(2.*self.real_x/25.) - 1. ,y_offset+(2.*self.real_y/14.) - 1.),
        });       
        ctx.draw(0, 6, 1);
        //SnakeDraw
        for b in &self.body_part
        {
            b.draw(ctx);
        }
    }

    fn compute_target(&mut self) {
        
        let new_dir = self.next_dir.pop().unwrap_or(self.dir);

        let last_x = self.curr_x;
        let last_y = self.curr_y;
        self.curr_x = self.dest_x;
        self.curr_y = self.dest_y;
        match new_dir {
            Dir::LEFT => 
            {   
                self.dir = Dir::LEFT;
                self.dest_x = self.curr_x - 1;
            },
            Dir::RIGHT => 
            {
                self.dir = Dir::RIGHT;
                self.dest_x = self.curr_x + 1;      
            },
            Dir::UP => 
            {
                self.dir = Dir::UP;
                self.dest_y = self.curr_y + 1;           
            },
            Dir::DOWN => 
            {
                self.dir = Dir::DOWN;
                self.dest_y = self.curr_y - 1;        
            }
        }

        //BODY TARGET
        let mut dest_x = self.curr_x;
        let mut dest_y = self.curr_y;
        for b in self.body_part.iter_mut()
        {
            if b.dest_x == -1 || b.dest_y == -1 {
                b.curr_x = last_x;
                b.curr_y = last_y;
            }
            else 
            {
                b.curr_x = b.dest_x;
                b.curr_y = b.dest_y;
            }
            b.dest_x = dest_x;
            b.dest_y = dest_y;
            dest_x = b.curr_x;
            dest_y = b.curr_y;
        }

    }

    pub(crate) fn eat_himself(&self) -> bool {        
        for b in &self.body_part
        {
            if b.curr_x == self.curr_x && b.curr_y == self.curr_y 
                && b.dest_x != -1 && b.dest_y != -1
            {
                eprintln!("EAT TAIL");
                return true;
            }
        }
        false
    }    
}


fn init_head_pipeline(ctx: &mut Context)  -> Pipeline{
    let shader = Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta()).unwrap();

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

fn init_head_bindings(ctx: &mut Context, dir: Dir)  -> Bindings {   
    
    let square_vertices: [Vertex; 4] = get_dir_vertex(&dir);
    
    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &square_vertices);
    
    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

    let texture = Texture::from_rgba8(ctx, 64, 64, &SNAKE_HEAD_RGB);

    Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer: index_buffer,
        images: vec![texture],
    }
}

fn get_dir_vertex(dir: &Dir) -> [Vertex; 4] {
    match dir {
        Dir::UP =>
        {
            [
                Vertex { pos : Vec2 { x: -32./800., y: -32./448. }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  32./800., y: -32./448. }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  32./800., y:  32./448.  }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x: -32./800., y:  32./448.  }, uv: Vec2 { x: 0., y: 1. } },
            ]
        },
        Dir::DOWN =>
        {
            [
                Vertex { pos : Vec2 { x: -32./800., y: -32./448. }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x:  32./800., y: -32./448. }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x:  32./800., y:  32./448.  }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x: -32./800., y:  32./448.  }, uv: Vec2 { x: 0., y: 0. } },
            ]
        },
        Dir::LEFT =>
        {
            [
                Vertex { pos : Vec2 { x: -32./800., y: -32./448. }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x:  32./800., y: -32./448. }, uv: Vec2 { x: 0., y: 0. } },
                Vertex { pos : Vec2 { x:  32./800., y:  32./448.  }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x: -32./800., y:  32./448.  }, uv: Vec2 { x: 1., y: 1. } },
            ]
        },
        Dir::RIGHT =>
        {
            [
                Vertex { pos : Vec2 { x: -32./800., y: -32./448. }, uv: Vec2 { x: 1., y: 0. } },
                Vertex { pos : Vec2 { x:  32./800., y: -32./448. }, uv: Vec2 { x: 1., y: 1. } },
                Vertex { pos : Vec2 { x:  32./800., y:  32./448.  }, uv: Vec2 { x: 0., y: 1. } },
                Vertex { pos : Vec2 { x: -32./800., y:  32./448.  }, uv: Vec2 { x: 0., y: 0. } },
            ]
        }
    }
}