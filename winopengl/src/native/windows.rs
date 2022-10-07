use std::ffi::CString;

use crate::{
    conf::Conf,
    native::NativeDisplayData,
    GraphicsContext, EventHandler,
};

use winapi::{
    shared::{
        minwindef::{DWORD, HIWORD, LPARAM, LRESULT, UINT, WPARAM},
        ntdef::{NULL},
        windef::{HDC, HWND, RECT},
    },
    um::{
        libloaderapi::{GetModuleHandleW, GetProcAddress},
        wingdi::*,
        winuser::*,
    },
};

mod keycodes;
mod libopengl32;
mod wgl;

use libopengl32::LibOpengl32;

pub(crate) struct Display {
    display_data: NativeDisplayData,
    content_scale: f32,
    window_scale: f32,
    libopengl32: LibOpengl32,
    _msg_wnd: HWND,
    msg_dc: HDC,
    wnd: HWND,
    dc: HDC,
}

impl crate::native::NativeDisplay for Display {
    fn order_quit(&mut self) {
        self.display_data.quit_ordered = true;
    }

    fn set_title(&mut self, title: String) {       
        let lp_text = CString::new(title).unwrap();

        unsafe {
            SetWindowTextA(
                self.wnd,
                lp_text.as_ptr(),
            );
        }
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
}

pub fn run<F>(conf: &Conf, f: F)
where
    F: 'static + FnOnce(&mut GraphicsContext) -> Box<dyn EventHandler>,
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
            content_scale: 1.,
            window_scale: 1.,
            display_data: Default::default(),
            libopengl32,
            _msg_wnd: msg_wnd,
            msg_dc,
            wnd,
            dc,
        };

        display.update_dimensions(wnd);
       // display.init_dpi(false);

        let mut wgl = wgl::Wgl::new(&mut display);
        let gl_ctx = wgl.create_context(
            &mut display,
            1,
            1,
        );

        super::gl::load_gl_funcs(|proc| display.get_proc_address(proc));

        let mut context = GraphicsContext::new(conf.window_width, conf.window_height);
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
        }
        (p.display.libopengl32.wglDeleteContext)(gl_ctx);
        DestroyWindow(wnd);
    }
}
