
#[derive(Debug)]
pub struct Conf {
    pub window_title: String,
    pub window_width: i32,
    pub window_height: i32,
/*
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub sample_count: i32,
    pub window_resizable: bool,

    high_dpi: false,
    fullscreen: false,
    sample_count: 1,
    window_resizable: true,
*/
}

// reasonable defaults for PC and mobiles are slightly different
#[cfg(not(target_os = "android"))]
impl Default for Conf {
    fn default() -> Conf {
        Conf {
            window_title: "".to_owned(),
            window_width: 800,
            window_height: 600,
        }
    }
}
