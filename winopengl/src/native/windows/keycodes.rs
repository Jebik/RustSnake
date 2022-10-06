use crate::event::KeyCode;

pub fn translate_keycode(keycode: u32) -> KeyCode {
    // same as GLFW
    match keycode {
        0x019 => KeyCode::P,
        0x001 => KeyCode::Escape,
        0x150 => KeyCode::Down,
        0x14B => KeyCode::Left,
        0x14D => KeyCode::Right,
        0x148 => KeyCode::Up,
        _ => KeyCode::Unknown,
    }
}
