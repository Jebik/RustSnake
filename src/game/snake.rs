use std::time::{SystemTime, Duration};

use miniquad::{Context};

use crate::{images::{SNAKE_HEAD, SNAKE_BODY}, pos::{Pos, FloatPos}, graphical_object::{GraphicalObject, ROTATION}};
use super::snake_body::SnakeBody;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Dir 
{
    Left,
    Right,
    Up,
    Down
}

pub struct Snake
{   
    //ForDrawing
    head: GraphicalObject,
    body: GraphicalObject,
    //ForLogic    
    pub pos: Pos,
    //DIRECTION
    pub dir: Dir,
    next_dir: Dir,
    //WEIRD STUFF
    body_part: Vec<Pos>,
    last_move_start: SystemTime,
}
impl Snake {
    pub(crate) fn new(ctx: &mut Context) -> Snake 
    {
        Snake
        {
            next_dir: Dir::None,
            body_part: Vec::new(),
            
            pos:Pos { x: 12, y: 7 },
            dir: Dir::Right,            
            last_move_start:SystemTime::now(),
            
            //ForDrawing
            body: GraphicalObject::new(ctx, SNAKE_BODY, true),
            head: GraphicalObject::new(ctx, SNAKE_HEAD, false)
        }
    }

    pub(crate) fn reset(&mut self) {
        self.body_part = Vec::new();
        self.next_dir = Dir::None;
        self.pos.x = 12;
        self.pos.y = 7;
        self.dir = Dir::Right;
    }    


    pub fn start(&mut self) {
        self. last_move_start = SystemTime::now();
    }

    pub fn check_reach(&mut self, move_duration: Duration)  -> bool
    {        
        let mut reach = false;
        let now = SystemTime::now();
        let curr_move_duration = now.duration_since(self.last_move_start).unwrap();

        if curr_move_duration > move_duration
        {   
            //we reach BIG EVENT
            reach = true;
            self.compute_target();
            self.last_move_start = SystemTime::now();    
        }

        reach
    }

    pub(crate) fn try_add(&mut self, dir: Dir) {
        match self.dir {
            Dir::Left => 
            {   
                if dir == Dir::Up || dir == Dir::Down
                {
                    self.next_dir = dir;
                }
            },
            Dir::Right => 
            {
                if dir == Dir::Up || dir == Dir::Down
                {
                    self.next_dir = dir;
                }                
            },
            Dir::Up => 
            {
                if dir == Dir::Left || dir == Dir::Right
                {
                    self.next_dir = dir;
                }                
            },
            Dir::Down => 
            {
                if dir == Dir::Left || dir == Dir::Right
                {
                    self.next_dir = dir;
                }                
            }
            Dir::None => todo!(),
        }
        
    }

    
    pub fn grow(&mut self) 
    {
        if self.body_part.len() == 0
        {
            self.body.start_shader_time();
        }
        self.body_part.push(Pos{ x:-1, y: -1});
    }
    
    pub fn draw(&mut self, ctx: &mut Context) {
        self.head.rotate(ctx, get_rotation(self.dir));
        self.head.draw(ctx, self.pos, 0.);
        let mut shader_time = 0.0f;
        //SnakeDraw
        for b in &self.body_part
        {
            self.body.draw(ctx, b.pos.x, shader_time);
            shader_time == 0.2f;
        }
    }

    fn compute_target(&mut self) {
        match dir {
            Dir::Left => 
            {   
                self.pos.x = self.pos.x - 1;
            },
            Dir::Right => 
            {
                self.pos.x = self.pos.x + 1;      
            },
            Dir::Up => 
            {
                self.pos.y = self.pos.y + 1;           
            },
            Dir::Down => 
            {
                self.pos.y = self.pos.y - 1;        
            }
            Dir::None => todo!(),
        }

        //BODY TARGET
        let last_x = self.pos.x;
        let last_y = self.pos.y;
        for b in self.body_part.iter_mut()
        {
            let curr_x = b.pos.x;
            let curr_y = b.pos.y;
            b.pos.x = last_x;
            b.pos.y = last_y;
            last_x = curr_x;
            last_y = curr_y;
        }
        self.dir = self.next_dir;
    }

    pub(crate) fn eat_himself(&self) -> bool {        
        for b in &self.body_part
        {
            if b.pos.x == self.pos.x && b.pos.y == self.pos.y
            {
                eprintln!("EAT TAIL");
                return true;
            }
        }
        false
    }    
}

fn get_rotation(dir: Dir) -> ROTATION {
    match dir {
        Dir::Left => ROTATION::Clockwise270,
        Dir::Right => ROTATION::Clockwise90,
        Dir::Up => ROTATION::None,
        Dir::Down => ROTATION::Clockwise180,
        Dir::None => ROTATION::None,
    }
}