use crate::Context;

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum KeyCode {
    P,
    Escape,
    Right,
    Left,
    Down,
    Up,
    Unknown,
}

pub trait EventHandler {
    fn update(&mut self, _ctx: &mut Context);
    fn draw(&mut self, _ctx: &mut Context);

    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode) {}
}
