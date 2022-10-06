/// Most backends happened to have exactly the same fields in their *Display struct
/// Maybe something like this may in some public API some day?
/// (important data from this struct is available through function like Context::screen_size)
#[allow(dead_code)]
pub(crate) struct NativeDisplayData {
    pub screen_width: i32,
    pub screen_height: i32,
    pub dpi_scale: f32,
    pub high_dpi: bool,
    pub quit_requested: bool,
    pub quit_ordered: bool,
}

impl Default for NativeDisplayData {
    fn default() -> NativeDisplayData {
        NativeDisplayData {
            screen_width: 1,
            screen_height: 1,
            dpi_scale: 1.,
            high_dpi: false,
            quit_requested: false,
            quit_ordered: false,
        }
    }
}

pub trait NativeDisplay: std::any::Any {
    fn screen_size(&self) -> (f32, f32);
    fn order_quit(&mut self);
    fn as_any(&mut self) -> &mut dyn std::any::Any;
}

pub mod module;

#[cfg(target_os = "windows")]
pub mod windows;

// there is no glGetProcAddr on webgl, so its impossible to make "gl" module work
// on macos.. well, there is, but way easier to just statically link to gl
#[cfg(not(any(target_arch = "wasm32", target_os = "macos")))]
pub mod gl;

pub mod query_stab;
