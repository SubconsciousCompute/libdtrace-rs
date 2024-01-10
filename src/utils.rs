#[derive(Debug)]
pub struct Error {
    _errno: i32,
    message: String,
}

impl From<::core::ffi::c_int> for Error {
    fn from(value: ::core::ffi::c_int) -> Self {
        let message = crate::wrapper::dtrace_hdl::dtrace_errmsg(None, value).to_string();
        Self { _errno: value, message }
    }
}

impl From<&crate::wrapper::dtrace_hdl> for Error {
    fn from(handle: &crate::wrapper::dtrace_hdl) -> Self {
        let errno = handle.dtrace_errno();
        let message = crate::wrapper::dtrace_hdl::dtrace_errmsg(Some(handle), errno).to_string();
        Self { _errno: errno, message }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl std::error::Error for Error {}

extern "C" {
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut crate::FILE;

    fn fclose(__stream: *mut crate::FILE) -> ::core::ffi::c_int;
}

pub struct File {
    pub file: *mut crate::FILE,
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            fclose(self.file);
        }
    }
}

impl File {
    pub fn new(filename: &str, modes: &str) -> Result<Self, String> {
        let filename = std::ffi::CString::new(filename).unwrap();
        let modes = std::ffi::CString::new(modes).unwrap();
        let file = unsafe { fopen(filename.as_ptr(), modes.as_ptr()) };
        if file.is_null() {
            Err("Failed to open file".to_string())
        } else {
            Ok(Self { file})
        }
    }
}
