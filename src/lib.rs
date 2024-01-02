#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod callbacks;
mod wrapper;

#[cfg(test)]
mod tests {
    use crate::*;
    use wrapper::dtrace_hdl;
    #[test]
    fn dtrace_get_handle() {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0);
        match handle {
            Ok(_) => {}
            Err(errno) => {
                let msg = dtrace_hdl::dtrace_errmsg(None, errno);
                panic!("{}", msg);
            }
        }
    }

    #[test]
    fn dtrace_set_option() {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0).unwrap();
        let status = handle.dtrace_setopt("bufsize", "4m");
        match status {
            Ok(_) => {}
            Err(errno) => {
                let msg = dtrace_hdl::dtrace_errmsg(Some(handle), errno);
                panic!("{}", msg);
            }
        }
    }

    #[test]
    fn dtrace_handle_buffered() {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0).unwrap();
        let status = handle.dtrace_handle_buffered(Some(callbacks::buffered));
        match status {
            Ok(_) => {}
            Err(errno) => {
                let msg = dtrace_hdl::dtrace_errmsg(Some(handle), errno);
                panic!("{}", msg);
            }
        }
    }

    #[test]
    fn dtrace_compile_program() {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0).unwrap();
        let status = handle
                    .dtrace_program_strcompile(
                        "dtrace:::BEGIN {trace(\"Hello World\");} syscall:::entry { @num[execname] = count(); }", 
                        dtrace_probespec::DTRACE_PROBESPEC_NAME, 
                        DTRACE_C_ZDEFS,
                        None);
        match status {
            Ok(_) => {}
            Err(errno) => {
                let msg = dtrace_hdl::dtrace_errmsg(Some(handle), errno);
                panic!("{}", msg);
            }
        }
    }
}
