use miniquad::{EventHandler, Context, KeyCode, KeyMods, Bindings, Shader, Pipeline, BufferLayout, VertexAttribute, VertexFormat, Buffer, BufferType, Texture};
use rand::Rng;

mod bonus;
mod snake;
mod snake_body;
use crate::{shader::{*, shader::*}, images::snake_body::SNAKE_BODY_RGB}; //images::snake_bg::SNAKE_BG_RGB};
use self::{snake::{Snake, Dir}, bonus::Bonus};


/* 
GAME INFO
1600x896
25x14 BOARD 64px CaseSize
*/

pub(crate) struct Game 
{
    snake:Snake,
    bonus:Bonus,
    score:i32,
    running:bool,
    width:i16,
    height:i16,
    pipeline: Pipeline,
    bindings: Bindings
}
impl Game
{
    pub(crate) fn new(ctx: &mut Context) -> Game
    {
        let mut g = Game
        {
            snake: Snake::new(ctx),
            bonus: Bonus::new(ctx),
            width: 25,
            height:14,
            pipeline: init_bg_pipeline(ctx),
            bindings: init_bg_bindings(ctx),
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
        self.bonus.x = x;
        self.bonus.y = y;
    }

    fn real_game_update(&mut self, ctx: &mut Context) 
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
        if self.snake.curr_x == self.bonus.x 
            && self.snake.curr_y == self.bonus.y
        {
            //We got apple
            self.snake.grow(ctx);
            self.spawn_bonus();
        }
    }

    fn check_game_over(&mut self) {
        if self.snake.dest_x < 0 || self.snake.dest_x > self.width 
            || self.snake.dest_y < 0 || self.snake.dest_y > self.height 
            || self.snake.eat_himself()
        {
            eprintln!("GAME OVER");
            self.init();
        }
    }
}


//OPENGL WEIRD
fn init_bg_pipeline(ctx: &mut Context)  -> Pipeline{
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

fn init_bg_bindings(ctx: &mut Context)  -> Bindings {    
    let bg_vertices: [Vertex; 4] = [
        Vertex { pos : Vec2 { x: -1., y: -1. }, uv: Vec2 { x: 0., y: 0. } },
        Vertex { pos : Vec2 { x:  1., y: -1. }, uv: Vec2 { x: 1., y: 0. } },
        Vertex { pos : Vec2 { x:  1., y:  1. }, uv: Vec2 { x: 1., y: 1. } },
        Vertex { pos : Vec2 { x: -1., y:  1. }, uv: Vec2 { x: 0., y: 1. } },
    ];

    let vertex_buffer = Buffer::immutable(ctx, BufferType::VertexBuffer, &bg_vertices);
    
    let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
    let index_buffer = Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

    let texture = Texture::from_rgba8(ctx, 64, 64, &SNAKE_BODY_RGB);
    //let texture = Texture::from_rgba8(ctx, 1600, 896, &SNAKE_BG_RGB);

    Bindings {
        vertex_buffers: vec![vertex_buffer],
        index_buffer: index_buffer,
        images: vec![texture],
    }
}

impl EventHandler for Game 
{
    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) 
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
                KeyCode::Escape => _ctx.quit(), 
                _ => ()             
            }   
        }
    }

    fn update(&mut self, ctx: &mut Context) 
    { 
        if self.running
        {
            self.real_game_update(ctx);
        }
    }

    fn draw(&mut self, ctx: &mut Context) 
    {
        /*let t = date::now();
        eprintln!("{:#?}", t);*/
        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&shader::Uniforms {
            offset: (0.,0.),
        });       
        ctx.draw(0, 6, 1);


        //SnakeDraw
        self.snake.draw(ctx);
        self.bonus.draw(ctx);

        ctx.end_render_pass();

        ctx.commit_frame();
    }
}