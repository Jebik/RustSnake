use std::{ffi::CString, time::Duration};

use winopengl::{EventHandler, GraphicsContext, KeyCode};
use rand::Rng;
use winapi::um::winuser::{MessageBoxA, MB_OK, MB_ICONINFORMATION, SetWindowTextA};

mod bonus;
mod snake;
mod background;
use crate::pos::Pos;

use self::{snake::{Snake, Dir}, bonus::Bonus, background::Background};

use crate::images::jpg as Images;
pub const BOX_SIZE: i16 = 64;
pub const SCREEN_WIDTH: i16 = 1600;
pub const SCREEN_HEIGHT: i16 = 896;
pub const SCREEN_WIDTH_FLOAT: f32 = 1600.;
pub const SCREEN_HEIGHT_FLOAT: f32 = 896.;

#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum DifficultyLevel 
{
    Easy,
    Medium,
    Hard,
    Insane
}

#[derive(Clone, Copy)]
pub struct Difficulty 
{
    move_duration: Duration,
    score_per_bonus:i32,
    bonus_count:i16,
    level:DifficultyLevel,
    next_level_trigger:i32
}

pub(crate) struct Game 
{
    snake:Snake,
    difficulty: Difficulty,
    bonus:Bonus,
    bg: Background,
    bonus_list: Vec<Pos>,
    score:i32,
    running:bool,
    width:i16,
    height:i16,
}
impl Game
{
    pub(crate) fn new(ctx: &mut GraphicsContext) -> Game
    {
        let mut g = Game
        {
            snake: Snake::new(ctx),
            difficulty: get_difficulty(DifficultyLevel::Easy),
            bonus: Bonus::new(ctx),
            bg: Background::new(ctx),
            bonus_list : Vec::new(),
            width: SCREEN_WIDTH/BOX_SIZE,
            height: SCREEN_HEIGHT/BOX_SIZE,
            score: 0,
            running: false
        };
        g.init();
        g
    }

    fn init(&mut self) {
        self.snake.reset();
        self.score = 0;
        self.running = false;
        self.difficulty = get_difficulty(DifficultyLevel::Easy);
        self.bonus_list = Vec::new();
        self.spawn_bonus();
    }

    fn spawn_bonus(&mut self){
        while self.bonus_list.len() < self.difficulty.bonus_count as _
        {
            let mut rng = rand::thread_rng();
            let x:i16 = rng.gen_range(0..self.width);
            let y:i16 = rng.gen_range(0..self.height);
            self.bonus_list.push(Pos{ x, y });
        }
    }

    fn real_game_update(&mut self) 
    {
        //MovingSnake and Checking if reach a case
        let reach = self.snake.check_reach(self.difficulty.move_duration);

        if !reach
        {
            return;
        }
        //Check if game over.
        self.check_game_over();
        //Check if on bonus
        for i in 0.. self.bonus_list.len()
        {
            let b = self.bonus_list[i];
            if self.snake.pos.x == b.x 
                && self.snake.pos.y == b.y
            {
                //We got apple
                self.score += self.difficulty.score_per_bonus;
                self.snake.start();
                self.update_title();
                self.bonus_list.remove(i);
                self.get_new_difficulty();
                self.snake.grow();
                self.spawn_bonus();
                break;
            }
        }
    }

    fn check_game_over(&mut self) {
        if self.snake.pos.x < 0 || self.snake.pos.x > self.width 
            || self.snake.pos.y < 0 || self.snake.pos.y > self.height 
            || self.snake.eat_himself()
        {
            self.running = false;            
            show_score(self.score);
            self.init();
        }
    }

    fn get_new_difficulty(&mut self) {

        let new_difficulty = match self.difficulty.level {
            DifficultyLevel::Easy => DifficultyLevel::Medium,
            DifficultyLevel::Medium => DifficultyLevel::Hard,
            DifficultyLevel::Hard => DifficultyLevel::Insane,
            _ =>  self.difficulty.level
        };
        if self.score > self.difficulty.next_level_trigger
        {
            eprintln!("Passing to={:?}", new_difficulty);
            self.difficulty = get_difficulty(new_difficulty);
        }
    }

    fn update_title(&self) {
        //WIN API MESSAGE SCORE
        let mut title = "AmbuSnake".to_owned();
        title += " Score: ";
        title += &self.score.to_string();
    
        let lp_text = CString::new(title).unwrap();
        unsafe {
            SetWindowTextA(
                std::ptr::null_mut(),
                lp_text.as_ptr(),
            );
        }
    }
}

fn get_difficulty(level: DifficultyLevel) -> Difficulty {
    match  level {
        DifficultyLevel::Easy => Difficulty
        {
            move_duration: Duration::from_millis(400),
            score_per_bonus: 1,
            bonus_count: 4,
            level: DifficultyLevel::Easy,
            next_level_trigger:10
        },
        DifficultyLevel::Medium =>Difficulty
        {
            move_duration: Duration::from_millis(300),
            score_per_bonus: 4,
            bonus_count: 3,
            level: DifficultyLevel::Medium,
            next_level_trigger:50
        },
        DifficultyLevel::Hard => Difficulty
        {
            move_duration: Duration::from_millis(200),
            score_per_bonus: 6,
            bonus_count: 2,
            level: DifficultyLevel::Hard,
            next_level_trigger:100
        },
        DifficultyLevel::Insane => Difficulty
        {
            move_duration: Duration::from_millis(100),
            score_per_bonus: 10,
            bonus_count: 1,
            level: DifficultyLevel::Insane,
            next_level_trigger:9999999
        },
    }
}

fn show_score(score: i32) {
    //WIN API MESSAGE SCORE
    let mut message_body = "Vous avez perdu\n\n".to_owned();
    message_body += "Score: ";
    message_body += &score.to_string();

    let lp_text = CString::new(message_body).unwrap();
    let lp_caption = CString::new("GAME OVER").unwrap();
    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            MB_OK | MB_ICONINFORMATION
        );
    }
}

impl EventHandler for Game 
{
    fn key_down_event(&mut self, ctx: &mut GraphicsContext, _keycode: KeyCode) 
    {
        if _keycode == KeyCode::Escape
        { 
            ctx.order_quit()
        }
        //On attend un premier input pour pas lancer tout de suite le jeu
        if !self.running
        {
            match _keycode {
                KeyCode::Up => self.running = true,
                KeyCode::Left => self.running = true,
                KeyCode::Down => self.running = true,
                KeyCode::Right => self.running = true,                
                _ => ()      
            }
        }
        else
        {
            match _keycode {
                KeyCode::Up => self.snake.try_add(Dir::Up),
                KeyCode::Left => self.snake.try_add(Dir::Left),
                KeyCode::Down => self.snake.try_add(Dir::Down),
                KeyCode::Right => self.snake.try_add(Dir::Right),
                KeyCode::P => self.running = false,
                _ => ()             
            }   
        }
    }

    fn update(&mut self, _ctx: &mut GraphicsContext) 
    { 
        if self.running
        {
            self.real_game_update();
        }
    }

    fn draw(&mut self, ctx: &mut GraphicsContext) 
    {
        ctx.begin_default_pass();

        self.bg.draw(ctx);

        //SnakeDraw
        self.snake.draw(ctx);
        for b in &self.bonus_list
        {
            self.bonus.draw(ctx, *b);
        }

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}