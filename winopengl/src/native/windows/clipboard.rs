use winapi::um::winuser::CloseClipboard;

struct ClipboardGuard;
impl ClipboardGuard {
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        unsafe {
            CloseClipboard();
        }
    }
}