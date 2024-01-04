#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
mod callbacks;
mod wrapper;
mod utils;

#[cfg(test)]
mod tests {
    use crate::*;
    use wrapper::dtrace_hdl;
    #[test]
    fn dtrace_get_handle() {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0);
        match handle {
            Ok(_) => {}
            Err(error) => {
                panic!("{}", error);
            }
        }
    }

    #[test]
    fn dtrace_set_option() {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0).unwrap();
        let status = handle.dtrace_setopt("bufsize", "4m");
        match status {
            Ok(_) => {}
            Err(error) => {
                panic!("{}", error);
            }
        }
    }

    #[test]
    fn dtrace_handle_buffered() {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0).unwrap();
        let status = handle.dtrace_handle_buffered(Some(callbacks::buffered), None);
        match status {
            Ok(_) => {}
            Err(error) => {
                panic!("{}", error);
            }
        }
    }

    #[test]
    fn dtrace_compile_and_exec() {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0).unwrap();
        let prog = handle
                    .dtrace_program_strcompile(
                        "dtrace:::BEGIN {trace(\"Hello World\");} syscall:::entry { @num[execname] = count(); }", 
                        dtrace_probespec::DTRACE_PROBESPEC_NAME, 
                        DTRACE_C_ZDEFS,
                        None);
        match prog {
            Ok(prog) => {
                let status = handle.dtrace_program_exec(&mut *prog, None);
                match status {
                    Ok(_) => {}
                    Err(error) => {
                        panic!("{}", error);
                    }
                }
            }
            Err(error) => {
                panic!("{}", error);
            }
        }                        
    }
}
