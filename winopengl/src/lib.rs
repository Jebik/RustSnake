pub mod conf;
mod event;
pub mod graphics;pub mod native;
pub use event::*;
pub use graphics::*;
pub use native::{gl, NativeDisplay};
pub use graphics::GraphicsContext as Context;

pub mod date {
    pub fn now() -> f64 {
        use std::time::SystemTime;

        let time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_else(|e| panic!("{}", e));
        time.as_secs_f64()
    }
}
impl Context {
    // Updates the display pointer inside the Context
    // Context should always be passed to event handlers through "with_display"
    pub(crate) fn with_display(&mut self, display: &mut dyn NativeDisplay) -> &mut Context {
        self.display = Some(display);
        self
    }
    pub fn display(&self) -> &dyn NativeDisplay {
        unsafe { &*self.display.unwrap() }
    }
    pub fn display_mut(&mut self) -> &mut dyn NativeDisplay {
        unsafe { &mut *self.display.unwrap() }
    }
    pub fn screen_size(&self) -> (f32, f32) {
        self.display().screen_size()
    }
    pub fn order_quit(&mut self) {
        self.display_mut().order_quit();
    }
}

/// Start miniquad.
pub fn start<F>(conf: conf::Conf, f: F)
where
    F: 'static + FnOnce(&mut Context) -> Box<dyn EventHandler>,
{
    #[cfg(target_os = "windows")]
    {
        native::windows::run(&conf, f);
    }
}