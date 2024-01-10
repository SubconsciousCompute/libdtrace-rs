use libdtrace_rs::*;

fn main() {
    let handle = wrapper::dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as i32, 0).unwrap();
    handle.dtrace_setopt("bufsize", "4m").unwrap();
    handle.dtrace_setopt("aggsize", "4m").unwrap();
    handle
        .dtrace_register_handler(
            crate::types::dtrace_handler::Buffered(Some(callbacks::buffered)),
            None,
        )
        .unwrap();

    let file = utils::openf("examples/program.d", "r").unwrap();
    let prog = handle
        .dtrace_program_fcompile(Some(file), DTRACE_C_ZDEFS, None)
        .unwrap();
    handle.dtrace_program_exec(prog, None).unwrap();
    handle.dtrace_go().unwrap();

    let output = utils::openf("output.txt", "w").unwrap();
    match handle.dtrace_status().unwrap() {
        types::dtrace_status::Ok => {
            handle
                .dtrace_consume(Some(output), Some(callbacks::chew), Some(callbacks::chew_rec), None)
                .unwrap();
        }
        _ => {}
    }

    handle.dtrace_stop().unwrap();
}
