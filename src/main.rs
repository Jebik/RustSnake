use miniquad::conf::Conf;
mod graphical_object;
mod pos;
mod images;
mod game;

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
            .. Default::default()
        },
        |mut ctx| Box::new(game::Game::new(&mut ctx)),
    );
}