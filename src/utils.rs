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
