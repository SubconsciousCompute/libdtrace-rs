use crate::wrapper::dtrace_hdl;

#[derive(Debug)]
pub enum DtraceError {
    Errno(i32),
}

// Get message using dtrace_errmsg
impl std::fmt::Display for DtraceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let errno = match self {
            DtraceError::Errno(errno) => errno,
        };
        write!(f, "{}", dtrace_hdl::dtrace_errmsg(None, *errno))
    }
}

impl From<i32> for DtraceError {
    fn from(errno: i32) -> Self {
        DtraceError::Errno(errno)
    }
}

impl std::error::Error for DtraceError {}

extern "C" {
    pub fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut crate::FILE;
}

pub fn openf(filename: &str, modes: &str) -> Result<*mut crate::FILE, String> {
    let filename = std::ffi::CString::new(filename).unwrap();
    let modes = std::ffi::CString::new(modes).unwrap();
    let file = unsafe { fopen(filename.as_ptr(), modes.as_ptr()) };
    if file.is_null() {
        Err("Failed to open file".to_string())
    } else {
        Ok(file)
    }
}
