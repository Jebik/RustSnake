use std::time::{SystemTime, Duration};

use miniquad::{Context};

use crate::{images::{SNAKE_HEAD, SNAKE_BODY}, pos::{Pos, FloatPos}, graphical_object::{GraphicalObject, ROTATION, self}};
use super::snake_body::SnakeBody;

#[derive(PartialEq, Clone, Copy)]
pub enum Dir
{
    LEFT,
    RIGHT,
    UP,
    DOWN,
    NONE
}

pub struct Snake
{   
    //ForDrawing
    pub real: FloatPos,
    head: GraphicalObject,
    body: GraphicalObject,
    //ForLogic
    pub curr: Pos,
    pub dest: Pos,
    //DIRECTION
    pub dir: Dir,
    next_dir: Dir,
    //WEIRD STUFF
    body_part:Vec<SnakeBody>,
    last_move_start: SystemTime,
}
impl Snake {
    pub(crate) fn new(ctx: &mut Context) -> Snake 
    {
        Snake
        {
            next_dir: Dir::NONE,
            body_part: Vec::new(),
            
            real:FloatPos { x: 12., y: 7. },
            curr:Pos { x: 12, y: 7 },
            dest:Pos { x: 13, y: 7 },
            dir: Dir::RIGHT,            
            last_move_start:SystemTime::now(),
            
            //ForDrawing
            body: GraphicalObject::new(ctx, SNAKE_BODY),
            head: GraphicalObject::new(ctx, SNAKE_HEAD)
        }
    }

    pub(crate) fn reset(&mut self) -> () {
        self.body_part = Vec::new();
        self.next_dir = Dir::NONE;
        self.real.x = 12.;
        self.real.y = 7.;
        self.curr.x = 12;
        self.curr.y = 7;
        self.dest.x = 13;
        self.dest.y = 7;
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
            self.real.x = f32::from(&self.dest.x-&self.curr.x)*ratio + f32::from(self.curr.x);
            self.real.y = f32::from(&self.dest.y-&self.curr.y)*ratio + f32::from(self.curr.y);
            
            for b in self.body_part.iter_mut()
            {
                if b.dest.x != -1 && b.dest.y != -1
                {
                    b.real.x = f32::from(&b.dest.x-&b.curr.x)*ratio + f32::from(b.curr.x);
                    b.real.y = f32::from(&b.dest.y-&b.curr.y)*ratio + f32::from(b.curr.y);
                }
            }
        }

        //return true if reach dest
        reach
    }

    pub(crate) fn try_add(&mut self, dir: Dir) {
        if self.next_dir != Dir::NONE
        {
            return;
        }
        
        match self.dir {
            Dir::LEFT => 
            {   
                if dir == Dir::UP || dir == Dir::DOWN
                {
                    self.next_dir = dir;
                }
            },
            Dir::RIGHT => 
            {
                if dir == Dir::UP || dir == Dir::DOWN
                {
                    self.next_dir = dir;
                }                
            },
            Dir::UP => 
            {
                if dir == Dir::LEFT || dir == Dir::RIGHT
                {
                    self.next_dir = dir;
                }                
            },
            Dir::DOWN => 
            {
                if dir == Dir::LEFT || dir == Dir::RIGHT
                {
                    self.next_dir = dir;
                }                
            }
            Dir::NONE => todo!(),
        }
        
    }

    
    pub fn grow(&mut self) 
    {
        let mut x = self.curr.x;
        let mut y = self.curr.y;

        let last_body = self.body_part.last();
        if last_body.is_some()
        {
            let body = last_body.unwrap();
            x = body.curr.x;
            y = body.curr.y;
        }

        self.body_part.push(SnakeBody::new(x, y))
    }

    
    pub fn draw(&mut self, ctx: &mut Context) {
        self.head.rotate(ctx, get_rotation(self.dir));
        self.head.draw(ctx, self.real.x, self.real.y);
        //SnakeDraw
        for b in &self.body_part
        {
            self.body.draw(ctx, b.real.x, b.real.y);
        }
    }

    fn compute_target(&mut self) {
        
        let new_dir = if self.next_dir != Dir::NONE {self.next_dir} else {self.dir};

        let last_x = self.curr.x;
        let last_y = self.curr.y;
        self.curr.x = self.dest.x;
        self.curr.y = self.dest.y;
        match new_dir {
            Dir::LEFT => 
            {   
                self.dir = Dir::LEFT;
                self.dest.x = self.curr.x - 1;
            },
            Dir::RIGHT => 
            {
                self.dir = Dir::RIGHT;
                self.dest.x = self.curr.x + 1;      
            },
            Dir::UP => 
            {
                self.dir = Dir::UP;
                self.dest.y = self.curr.y + 1;           
            },
            Dir::DOWN => 
            {
                self.dir = Dir::DOWN;
                self.dest.y = self.curr.y - 1;        
            }
            Dir::NONE => todo!(),
        }

        //BODY TARGET
        let mut dest_x = self.curr.x;
        let mut dest_y = self.curr.y;
        for b in self.body_part.iter_mut()
        {
            if b.dest.x == -1 || b.dest.y == -1 {
                b.curr.x = last_x;
                b.curr.y = last_y;
            }
            else 
            {
                b.curr.x = b.dest.x;
                b.curr.y = b.dest.y;
            }
            b.dest.x = dest_x;
            b.dest.y = dest_y;
            dest_x = b.curr.x;
            dest_y = b.curr.y;
        }

    }

    pub(crate) fn eat_himself(&self) -> bool {        
        for b in &self.body_part
        {
            if b.curr.x == self.curr.x && b.curr.y == self.curr.y 
                && b.dest.x != -1 && b.dest.y != -1
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
        Dir::LEFT => ROTATION::Clockwise270,
        Dir::RIGHT => ROTATION::Clockwise90,
        Dir::UP => ROTATION::NONE,
        Dir::DOWN => ROTATION::Clockwise180,
        Dir::NONE => ROTATION::NONE,
    }
}