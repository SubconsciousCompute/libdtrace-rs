use libdtrace_rs::*;

fn main() {
    let handle = wrapper::dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as i32, 0).unwrap();
    handle.dtrace_setopt("bufsize", "4m").unwrap();
    handle.dtrace_setopt("aggsize", "4m").unwrap();
    handle
        .dtrace_handle_buffered(Some(callbacks::buffered), None)
        .unwrap();
    let prog = handle
        .dtrace_program_strcompile(
            "BEGIN {trace(\"Hello World\");}",
            dtrace_probespec::DTRACE_PROBESPEC_NAME,
            DTRACE_C_ZDEFS,
            None,
        )
        .unwrap();
    handle.dtrace_program_exec(prog, None).unwrap();
    handle.dtrace_go().unwrap();

    match handle.dtrace_status().unwrap() {
        wrapper::dtrace_status::Ok => {
            handle
                .dtrace_consume(
                    None, 
                    Some(callbacks::chew), 
                    Some(callbacks::chew_rec), 
                    None
                ).unwrap();
        }
        _ => {}
    }

    handle.dtrace_aggregate_print(None, None).unwrap();
    handle.dtrace_stop().unwrap();
}