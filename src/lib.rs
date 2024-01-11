#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
pub mod callbacks;
pub mod wrapper;
pub mod utils;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::*;
    use wrapper::dtrace_hdl;
    #[test]
    fn dtrace_get_handle() -> Result<(), utils::Error> {
        dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0)?;
        Ok(())
    }

    #[test]
    fn dtrace_set_option() -> Result<(), utils::Error> {
        let result = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0)?
                    .dtrace_setopt("bufsize", "4m")?;
        assert_eq!(result.dtrace_getopt("bufsize")?, 4194304);
        Ok(())
    }

    #[test]
    fn dtrace_handle_buffered() -> Result<(), utils::Error> {
        dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0)?
            .dtrace_register_handler(crate::types::dtrace_handler::Buffered(Some(callbacks::buffered)), None)?;
        Ok(())
    }

    #[test]
    fn dtrace_compile_and_exec() -> Result<(), utils::Error> {
        let handle = dtrace_hdl::dtrace_open(DTRACE_VERSION as i32, 0)?;
        let prog = handle
                    .dtrace_program_strcompile(
                        "dtrace:::BEGIN {trace(\"Hello World\");} syscall:::entry { @num[execname] = count(); }", 
                        dtrace_probespec::DTRACE_PROBESPEC_NAME, 
                        DTRACE_C_ZDEFS,
                        None)?;
        
        handle.dtrace_program_exec(prog, None)?;

        Ok(())
    }
}
