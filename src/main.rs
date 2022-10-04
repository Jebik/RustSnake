
mod shader;
mod images;
mod game;
use miniquad::conf::{Icon, Conf};
//use images::snake_bg::SNAKE_BG_RGB;
use images::snake_body::SNAKE_BODY_RGB;
use images::snake_body_32::SNAKE_BODY_32_RGB;
use images::snake_body_16::SNAKE_BODY_16_RGB;

pub fn samu_icon() -> Icon {
    Icon{
        small: SNAKE_BODY_16_RGB,
        medium: SNAKE_BODY_32_RGB,
        big: SNAKE_BODY_RGB
    }
}

//CREATE LOGICAL OBJECT
//CUR POS
//PREV POS
//CREATE Struct POS
//X et Y

//CREATE GRAPHICAL OBJECT
//INTERNAL PIPELINE
//INTERNAL BINDING
//INTERNAL ROTATION
//PUB DRAW(X, Y)

//Create My miniquad
//REMOVE ALL NON WINDOWS
//REMOVE ALL UNUSED
//SIMPLIFIE A LOT

fn main() {
    miniquad::start(
        Conf {
            window_title: "AmbuSnake".to_string(),
            window_width: 1600,
            window_height: 896,
            window_resizable: false,
            fullscreen: false,
            icon: Some(samu_icon()),
            .. Default::default()
        },
        |mut ctx| Box::new(game::Game::new(&mut ctx)),
    );
}