use std::ffi::CString;

use miniquad::{EventHandler, Context, KeyCode, KeyMods};
use rand::Rng;
use winapi::um::winuser::{MessageBoxA, MB_OK, MB_ICONINFORMATION};

mod bonus;
mod snake;
mod background;
mod snake_body;
use self::{snake::{Snake, Dir}, bonus::Bonus, background::Background};

/* 
GAME INFO
1600x896
25x14 BOARD 64px CaseSize
*/

pub(crate) struct Game 
{
    snake:Snake,
    bonus:Bonus,
    bg: Background,
    score:i32,
    running:bool,
    width:i16,
    height:i16,
}
impl Game
{
    pub(crate) fn new(ctx: &mut Context) -> Game
    {
        let mut g = Game
        {
            snake: Snake::new(ctx),
            bonus: Bonus::new(ctx),
            bg: Background::new(ctx),
            width: 25,
            height:14,
            score: 0,
            running: false
        };
        g.init();
        g
    }

    fn init(&mut self) -> () {
        self.snake.reset();
        self.score = 0;
        self.running = false;
        self.spawn_bonus();
    }

    fn spawn_bonus(&mut self) -> () {
        let mut rng = rand::thread_rng();
        let x:i16 = rng.gen_range(0..self.width);
        let y:i16 = rng.gen_range(0..self.height);
        self.bonus.pos.x = x;
        self.bonus.pos.y = y;
    }

    fn real_game_update(&mut self) 
    {
        //MovingSnake and Checking if reach a case
        let reach = self.snake.check_reach();

        if !reach
        {
            return;
        }
        //Check if game over.
        self.check_game_over();
        //Check if on bonus
        if self.snake.curr.x == self.bonus.pos.x 
            && self.snake.curr.y == self.bonus.pos.y
        {
            //We got apple
            self.score += 1;
            self.snake.grow();
            self.spawn_bonus();
        }
    }

    fn check_game_over(&mut self) {
        if self.snake.dest.x < 0 || self.snake.dest.x > self.width 
            || self.snake.dest.y < 0 || self.snake.dest.y > self.height 
            || self.snake.eat_himself()
        {
            show_score(self.score);
            self.running = false;
            self.init();
        }
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
    fn key_up_event(&mut self, ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) 
    {
        //On attend un premier input pour pas lancer tout de suite le jeu
        if !self.running
        {
            self.running = true;
            self.snake.start();
        }
        else
        {
            match _keycode {
                KeyCode::Up => self.snake.try_add(Dir::UP),
                KeyCode::Left => self.snake.try_add(Dir::LEFT),
                KeyCode::Down => self.snake.try_add(Dir::DOWN),
                KeyCode::Right => self.snake.try_add(Dir::RIGHT),
                KeyCode::P => self.running = false,
                KeyCode::Escape => ctx.quit(), 
                _ => ()             
            }   
        }
    }

    fn update(&mut self, _ctx: &mut Context) 
    { 
        if self.running
        {
            self.real_game_update();
        }
    }

    fn draw(&mut self, ctx: &mut Context) 
    {
        ctx.begin_default_pass(Default::default());

        self.bg.draw(ctx);

        //SnakeDraw
        self.snake.draw(ctx);
        self.bonus.draw(ctx);

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}