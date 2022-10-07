/// Most backends happened to have exactly the same fields in their *Display struct
/// Maybe something like this may in some public API some day?
/// (important data from this struct is available through function like Context::screen_size)
pub(crate) struct NativeDisplayData {
    pub quit_ordered: bool,
}

impl Default for NativeDisplayData {
    fn default() -> NativeDisplayData {
        NativeDisplayData {
            quit_ordered: false,
        }
    }
}

pub trait NativeDisplay: std::any::Any {
    fn set_title(&mut self, title: String);
    fn order_quit(&mut self);
    fn as_any(&mut self) -> &mut dyn std::any::Any;
}

pub mod module;
pub mod windows;
pub mod gl;
