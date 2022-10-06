use crate::{
    conf::Conf,
    native::NativeDisplayData,
    Context, EventHandler, GraphicsContext,
};

use winapi::{
    shared::{
        minwindef::{DWORD, HINSTANCE, HIWORD, LPARAM, LRESULT, UINT, WPARAM},
        ntdef::{HRESULT, NULL},
        windef::{HDC, HMONITOR, HWND, POINT, RECT},
    },
    um::{
        libloaderapi::{FreeLibrary, GetModuleHandleW, GetProcAddress, LoadLibraryA},
        shellscalingapi::*,
        wingdi::*,
        winuser::*,
    },
};

mod clipboard;
mod keycodes;
mod libopengl32;
mod wgl;

use libopengl32::LibOpengl32;

pub(crate) struct Display {
    fullscreen: bool,
    dpi_aware: bool,
    window_resizable: bool,
    cursor_grabbed: bool,
    display_data: NativeDisplayData,
    content_scale: f32,
    window_scale: f32,
    mouse_scale: f32,
    libopengl32: LibOpengl32,
    _msg_wnd: HWND,
    msg_dc: HDC,
    wnd: HWND,
    dc: HDC,
}

impl crate::native::NativeDisplay for Display {
    fn screen_size(&self) -> (f32, f32) {
        (
            self.display_data.screen_width as _,
            self.display_data.screen_height as _,
        )
    }
    fn order_quit(&mut self) {
        self.display_data.quit_ordered = true;
    }

    fn set_window_size(&mut self, new_width: u32, new_height: u32) {
        let mut x = 0;
        let mut y = 0;
        let mut final_new_width = new_width;
        let mut final_new_height = new_height;

        let mut rect: RECT = unsafe { std::mem::zeroed() };
        if unsafe { GetClientRect(self.wnd, &mut rect as *mut _ as _) } != 0 {
            x = rect.left;
            y = rect.bottom;
        }

        rect.right = (rect.left + new_width as i32) as _;
        rect.top = (rect.bottom - new_height as i32) as _;

        let win_style = get_win_style(self.fullscreen, self.window_resizable);
        let win_style_ex: DWORD = unsafe { GetWindowLongA(self.wnd, GWL_EXSTYLE) as _ };
        if unsafe {
            AdjustWindowRectEx(
                &mut rect as *mut _ as _,
                win_style,
                false as _,
                win_style_ex,
            )
        } != 0
        {
            final_new_width = (rect.right - rect.left) as _;
            final_new_height = (rect.bottom - rect.top) as _;
        }

        unsafe {
            SetWindowPos(
                self.wnd,
                HWND_TOP,
                x,
                y,
                final_new_width as i32,
                final_new_height as i32,
                SWP_NOMOVE,
            )
        };
    }
    fn as_any(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

struct WindowPayload {
    event_handler: Box<dyn EventHandler>,
    context: GraphicsContext,
    display: Display,
}

fn get_win_style(is_fullscreen: bool, is_resizable: bool) -> DWORD {
    if is_fullscreen {
        WS_POPUP | WS_SYSMENU | WS_VISIBLE
    } else {
        let mut win_style: DWORD =
            WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;

        if is_resizable {
            win_style |= WS_MAXIMIZEBOX | WS_SIZEBOX;
        }

        win_style
    }
}

unsafe fn update_clip_rect(hwnd: HWND) {
    // Retrieve the screen coordinates of the client area,
    // and convert them into client coordinates.
    let mut rect: RECT = std::mem::zeroed();

    GetClientRect(hwnd, &mut rect as *mut _ as _);
    let mut upper_left = POINT {
        x: rect.left,
        y: rect.top,
    };
    let mut lower_right = POINT {
        x: rect.right,
        y: rect.bottom,
    };

    ClientToScreen(hwnd, &mut upper_left as *mut _ as _);
    ClientToScreen(hwnd, &mut lower_right as *mut _ as _);

    SetRect(
        &mut rect as *mut _ as _,
        upper_left.x,
        upper_left.y,
        lower_right.x,
        lower_right.y,
    );
    ClipCursor(&mut rect as *mut _ as _);
}


unsafe extern "system" fn win32_wndproc(
    hwnd: HWND,
    umsg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let display_ptr = GetWindowLongPtrA(hwnd, GWLP_USERDATA);
    if display_ptr == 0 {
        return DefWindowProcW(hwnd, umsg, wparam, lparam);
    }
    let &mut WindowPayload {
        ref mut display,
        ref mut context,
        ref mut event_handler,
    } = &mut *(display_ptr as *mut WindowPayload);

    match umsg {
        WM_CLOSE => {
            PostQuitMessage(0);
            return 0;
        }
        WM_MOVE if display.cursor_grabbed => {
            update_clip_rect(hwnd);
        }

        WM_KEYDOWN | WM_SYSKEYDOWN => {
            let keycode = HIWORD(lparam as _) as u32 & 0x1FF;
            let keycode = keycodes::translate_keycode(keycode);
            event_handler.key_down_event(context.with_display(display), keycode);
        }
        _ => {}
    }

    DefWindowProcW(hwnd, umsg, wparam, lparam)
}

unsafe fn create_window(
    window_title: &str,
    width: i32,
    height: i32,
) -> (HWND, HDC) {
    let mut wndclassw: WNDCLASSW = std::mem::zeroed();

    wndclassw.style = CS_HREDRAW | CS_VREDRAW | CS_OWNDC;
    wndclassw.lpfnWndProc = Some(win32_wndproc);
    wndclassw.hInstance = GetModuleHandleW(NULL as _);
    wndclassw.hCursor = LoadCursorW(NULL as _, IDC_ARROW);
    wndclassw.hIcon = LoadIconW(NULL as _, IDI_WINLOGO);
    let class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();
    wndclassw.lpszClassName = class_name.as_ptr() as _;
    wndclassw.cbWndExtra = std::mem::size_of::<*mut std::ffi::c_void>() as i32;
    RegisterClassW(&wndclassw);

    let win_style: DWORD;
    let win_ex_style: DWORD = WS_EX_APPWINDOW | WS_EX_WINDOWEDGE;
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    win_style = WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX;

    rect.right = width;
    rect.bottom = height;

    AdjustWindowRectEx(&rect as *const _ as _, win_style, false as _, win_ex_style);
    let win_width = rect.right - rect.left;
    let win_height = rect.bottom - rect.top;
    let class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();
    let mut window_name = window_title.encode_utf16().collect::<Vec<u16>>();
    window_name.push(0);
    let hwnd = CreateWindowExW(
        win_ex_style,                // dwExStyle
        class_name.as_ptr(),         // lpClassName
        window_name.as_ptr(),        // lpWindowName
        win_style,                   // dwStyle
        CW_USEDEFAULT,               // X
        CW_USEDEFAULT,               // Y
        win_width,                   // nWidth
        win_height,                  // nHeight
        NULL as _,                   // hWndParent
        NULL as _,                   // hMenu
        GetModuleHandleW(NULL as _), // hInstance
        NULL as _,                   // lparam
    );
    assert!(hwnd.is_null() == false);
    ShowWindow(hwnd, SW_SHOW);
    let dc = GetDC(hwnd);
    assert!(dc.is_null() == false);

    (hwnd, dc)
}

unsafe fn create_msg_window() -> (HWND, HDC) {
    let class_name = "MINIQUADAPP\0".encode_utf16().collect::<Vec<u16>>();
    let window_name = "miniquad message window\0"
        .encode_utf16()
        .collect::<Vec<u16>>();
    let msg_hwnd = CreateWindowExW(
        WS_EX_OVERLAPPEDWINDOW,
        class_name.as_ptr() as _,
        window_name.as_ptr() as _,
        WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
        0,
        0,
        1,
        1,
        NULL as _,
        NULL as _,
        GetModuleHandleW(NULL as _),
        NULL,
    );
    assert!(
        msg_hwnd.is_null() == false,
        "Win32: failed to create helper window!"
    );
    ShowWindow(msg_hwnd, SW_HIDE);
    let mut msg = std::mem::zeroed();
    while PeekMessageW(&mut msg as _, msg_hwnd, 0, 0, PM_REMOVE) != 0 {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }
    let msg_dc = GetDC(msg_hwnd);
    assert!(
        msg_dc.is_null() == false,
        "Win32: failed to obtain helper window DC!"
    );

    (msg_hwnd, msg_dc)
}

impl Display {
    unsafe fn get_proc_address(&mut self, proc: &str) -> Option<unsafe extern "C" fn() -> ()> {
        let proc = std::ffi::CString::new(proc).unwrap();
        let mut proc_ptr = (self.libopengl32.wglGetProcAddress)(proc.as_ptr());
        if proc_ptr.is_null() {
            proc_ptr = GetProcAddress(self.libopengl32.module.0, proc.as_ptr());
        }
        assert!(
            proc_ptr.is_null() == false,
            "Load GL func {:?} failed.",
            stringify!($fn)
        );
        Some(std::mem::transmute(proc_ptr))
    }

    /// updates current window and framebuffer size from the window's client rect,
    /// returns true if size has changed
    unsafe fn update_dimensions(&mut self, hwnd: HWND) -> bool {
        let mut rect: RECT = std::mem::zeroed();

        if GetClientRect(hwnd, &mut rect as *mut _ as _) != 0 {
            let window_width = ((rect.right - rect.left) as f32 / self.window_scale) as i32;
            let window_height = ((rect.bottom - rect.top) as f32 / self.window_scale) as i32;
            let fb_width = (window_width as f32 * self.content_scale) as i32;
            let fb_height = (window_height as f32 * self.content_scale) as i32;
            if fb_width != self.display_data.screen_width
                || fb_height != self.display_data.screen_height
            {
                // prevent a framebuffer size of 0 when window is minimized
                self.display_data.screen_width = fb_width.max(1);
                self.display_data.screen_height = fb_height.max(1);
                return true;
            }
        } else {
            self.display_data.screen_width = 1;
            self.display_data.screen_height = 1;
        }
        return false;
    }

    unsafe fn init_dpi(&mut self, high_dpi: bool) {
        unsafe fn get_proc_address<T>(lib: HINSTANCE, proc: &[u8]) -> Option<T> {
            let proc = GetProcAddress(lib, proc.as_ptr() as *const _);

            if proc.is_null() {
                return None;
            }
            return Some(std::mem::transmute_copy(&proc));
        }

        let user32 = LoadLibraryA(b"user32.dll\0".as_ptr() as *const _);

        let mut setprocessdpiaware: Option<extern "system" fn() -> bool> = None;
        if user32.is_null() == false {
            setprocessdpiaware = get_proc_address(user32, b"SetProcessDPIAware\0");
        }

        let shcore = LoadLibraryA(b"shcore.dll\0".as_ptr() as *const _);

        let mut setprocessdpiawareness: Option<
            extern "system" fn(_: PROCESS_DPI_AWARENESS) -> HRESULT,
        > = None;
        let mut getdpiformonitor: Option<
            extern "system" fn(
                _: HMONITOR,
                _: MONITOR_DPI_TYPE,
                _: *mut UINT,
                _: *mut UINT,
            ) -> HRESULT,
        > = None;

        if shcore.is_null() == false {
            setprocessdpiawareness = get_proc_address(shcore, b"SetProcessDpiAwareness\0");
            getdpiformonitor = get_proc_address(shcore, b"GetDpiForMonitor\0");
        }

        if let Some(setprocessdpiawareness) = setprocessdpiawareness {
            // if the app didn't request HighDPI rendering, let Windows do the upscaling
            let mut process_dpi_awareness = PROCESS_SYSTEM_DPI_AWARE;
            self.dpi_aware = true;
            if !high_dpi {
                process_dpi_awareness = PROCESS_DPI_UNAWARE;
                self.dpi_aware = false;
            }
            setprocessdpiawareness(process_dpi_awareness);
        } else if let Some(setprocessdpiaware) = setprocessdpiaware {
            setprocessdpiaware();
            self.dpi_aware = true;
        }
        // get dpi scale factor for main monitor
        if let Some(getdpiformonitor) = getdpiformonitor {
            if self.dpi_aware {
                let pt = POINT { x: 1, y: 1 };
                let hm = MonitorFromPoint(pt, MONITOR_DEFAULTTONEAREST);
                let mut dpix: UINT = 0;
                let mut dpiy: UINT = 0;
                let hr = getdpiformonitor(
                    hm,
                    MDT_EFFECTIVE_DPI,
                    &mut dpix as *mut _ as _,
                    &mut dpiy as *mut _ as _,
                );
                assert_eq!(hr, 0);
                //  clamp window scale to an integer factor
                self.window_scale = dpix as f32 / 96.0;
            }
        } else {
            self.window_scale = 1.0;
        }
        if high_dpi {
            self.content_scale = self.window_scale;
            self.mouse_scale = 1.0;
        } else {
            self.content_scale = 1.0;
            self.mouse_scale = 1.0 / self.window_scale;
        }
        if !user32.is_null() {
            FreeLibrary(user32);
        }
        if !shcore.is_null() {
            FreeLibrary(shcore);
        }
    }
}

pub fn run<F>(conf: &Conf, f: F)
where
    F: 'static + FnOnce(&mut Context) -> Box<dyn EventHandler>,
{
    unsafe {
        let (wnd, dc) = create_window(
            &conf.window_title,
            conf.window_width as _,
            conf.window_height as _,
        );
        let libopengl32 = LibOpengl32::try_load().expect("Failed to load opengl32.dll.");

        let (msg_wnd, msg_dc) = create_msg_window();
        let mut display = Display {
            fullscreen: false,
            dpi_aware: false,
            window_resizable: false,
            cursor_grabbed: false,
            content_scale: 1.,
            mouse_scale: 1.,
            window_scale: 1.,
            display_data: Default::default(),
            libopengl32,
            _msg_wnd: msg_wnd,
            msg_dc,
            wnd,
            dc,
        };

        display.update_dimensions(wnd);
        display.init_dpi(false);

        let mut wgl = wgl::Wgl::new(&mut display);
        let gl_ctx = wgl.create_context(
            &mut display,
            1,
            1,
        );

        super::gl::load_gl_funcs(|proc| display.get_proc_address(proc));

        let mut context = GraphicsContext::new();
        context.features.instancing = !crate::gl::is_gl2();

        let event_handler = f(context.with_display(&mut display));

        let mut p = WindowPayload {
            display,
            context,
            event_handler,
        };
        // well, technically this is UB and we are suppose to use *mut WindowPayload instead of &mut WindowPayload forever from now on...
        // so if there going to be some weird bugs someday in the future - check this out!
        SetWindowLongPtrA(wnd, GWLP_USERDATA, &mut p as *mut _ as isize);

        let mut done = false;
        while !(done || p.display.display_data.quit_ordered) {
            let mut msg: MSG = std::mem::zeroed();
            while PeekMessageW(&mut msg as *mut _ as _, NULL as _, 0, 0, PM_REMOVE) != 0 {
                if WM_QUIT == msg.message {
                    done = true;
                    continue;
                } else {
                    TranslateMessage(&mut msg as *mut _ as _);
                    DispatchMessageW(&mut msg as *mut _ as _);
                }
            }
            p.event_handler.update(p.context.with_display(&mut p.display));
            p.event_handler.draw(p.context.with_display(&mut p.display));
            SwapBuffers(p.display.dc);
            if p.display.display_data.quit_requested {
                PostMessageW(p.display.wnd, WM_CLOSE, 0, 0);
            }
        }
        (p.display.libopengl32.wglDeleteContext)(gl_ctx);
        DestroyWindow(wnd);
    }
}
