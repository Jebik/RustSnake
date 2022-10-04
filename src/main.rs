use std::ffi::CString;

use miniquad::conf::Conf;
use winapi::um::winuser::{MessageBoxA, MB_ICONINFORMATION, MB_OK};
mod graphical_object;
mod pos;
mod images;
mod game;


//Create My miniquad
//REMOVE ALL NON WINDOWS
//REMOVE ALL UNUSED
//SIMPLIFIE A LOT

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