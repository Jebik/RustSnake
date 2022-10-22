use std::ffi::CString;

//#![no_std]
use game::{SCREEN_HEIGHT, SCREEN_WIDTH};
use winapi::um::winuser::{MessageBoxA, MB_ICONINFORMATION, MB_OK};
use winopengl::conf::Conf;
mod graphical_object;
mod texture;
mod pos;
mod images;
mod game;

fn main() {
    let mut message_body = "Vous avez trouver le snake cacher\n".to_owned();
    message_body += "Utilisez les fleches pour tourner\n";
    message_body += "Utilisez P pour mettre le jeu en pause\n";
    message_body += "Utilisez Echap pour quitter\n";

    let lp_text = CString::new(message_body).unwrap();
    let lp_caption = CString::new("Felicitation").unwrap();
    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            MB_OK | MB_ICONINFORMATION
        );
    }

    winopengl::start(
        Conf {
            window_title: "AmbuSnake".to_string(),
            window_width: SCREEN_WIDTH as _,
            window_height: SCREEN_HEIGHT as _
        },
        |ctx| Box::new(game::Game::new(ctx)),
    );
}