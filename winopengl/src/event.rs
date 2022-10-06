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
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct KeyMods {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

/// A trait defining event callbacks.
pub trait EventHandler {
    /// On most platforms update() and draw() are called each frame, sequentially,
    /// draw right after update.
    /// But on Android (and maybe some other platforms in the future) update might
    /// be called without draw.
    /// When the app is in background, Android destroys the rendering surface,
    /// while app is still alive and can do some usefull calculations.
    /// Note that in this case drawing from update may lead to crashes.
    fn update(&mut self, _ctx: &mut Context);
    fn draw(&mut self, _ctx: &mut Context);

    fn key_down_event(&mut self, _ctx: &mut Context, _keycode: KeyCode) {}
}
