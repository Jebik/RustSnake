#[derive(Debug)]
pub enum Error {
    DlOpenError,
    DlSymError,
}
#[cfg(target_os = "windows")]
mod windows {
    use super::Error;
    use winapi::{
        shared::minwindef::HINSTANCE,
        um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA},
    };

    pub struct Module(pub HINSTANCE);

    impl Module {
        pub fn load(path: &str) -> Result<Self, Error> {
            eprintln!("load {:?}", path);
            let path = std::ffi::CString::new(path).unwrap();
            let library = unsafe { LoadLibraryA(path.as_ptr()) };

            if library.is_null() {
                return Err(Error::DlOpenError);
            }
            Ok(Self(library))
        }
        pub fn get_symbol<F: Sized>(&self, name: &str) -> Result<F, Error> {
            eprintln!("get_symbol {:?}", name);
            let name = std::ffi::CString::new(name).unwrap();
            let proc = unsafe { GetProcAddress(self.0, name.as_ptr() as *const _) };

            if proc.is_null() {
                return Err(Error::DlSymError);
            }
            return Ok(unsafe { std::mem::transmute_copy(&proc) });
        }
    }

    impl Drop for Module {
        fn drop(&mut self) {
            unsafe { FreeLibrary(self.0) };
        }
    }
}

pub use windows::Module;
