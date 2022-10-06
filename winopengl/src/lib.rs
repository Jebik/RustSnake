pub mod conf;
mod event;
pub mod fs;
pub mod graphics;

pub mod native;

#[cfg(feature = "log-impl")]
pub mod log;

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

    /// The current framebuffer size in pixels
    /// NOTE: [High DPI Rendering](../conf/index.html#high-dpi-rendering)
    pub fn screen_size(&self) -> (f32, f32) {
        self.display().screen_size()
    }

    /// This function simply quits the application without
    /// giving the user a chance to intervene. Usually this might
    /// be called when the user clicks the 'Ok' button in a 'Really Quit?'
    /// dialog box
    /// Window might not be actually closed right away (exit(0) might not
    /// happen in the order_quit implmentation) and execution might continue for some time after
    /// But the window is going to be inevitably closed at some point.
    pub fn order_quit(&mut self) {
        self.display_mut().order_quit();
    }

    /// Set the application's window size.
    pub fn set_window_size(&mut self, new_width: u32, new_height: u32) {
        self.display_mut().set_window_size(new_width, new_height);
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum CursorIcon {
    Default,
    Help,
    Pointer,
    Wait,
    Crosshair,
    Text,
    Move,
    NotAllowed,
    EWResize,
    NSResize,
    NESWResize,
    NWSEResize,
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