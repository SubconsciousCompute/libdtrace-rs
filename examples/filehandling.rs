use libdtrace_rs::*;

fn main() -> Result<(), utils::Error> {
    let handle = wrapper::dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as i32, 0)?
        .dtrace_setopt("bufsize", "4m")?
        .dtrace_setopt("aggsize", "4m")?
        .dtrace_register_handler(
            crate::types::dtrace_handler::Buffered(Some(callbacks::buffered)),
            None,
        )?;

    let file = utils::File::new("examples/program.d", "r").unwrap();
    let prog = handle
        .dtrace_program_fcompile(Some(&file), DTRACE_C_ZDEFS, None)
        .unwrap();
    handle.dtrace_program_exec(prog, None).unwrap();
    handle.dtrace_go().unwrap();

    let output = utils::File::new("output.txt", "w").unwrap();
    match handle.dtrace_status().unwrap() {
        types::dtrace_status::Ok => {
            handle
                .dtrace_consume(
                    Some(&output),
                    Some(callbacks::chew),
                    Some(callbacks::chew_rec),
                    None,
                )
                .unwrap();
        }
        _ => {}
    }

    handle.dtrace_stop().unwrap();

    Ok(())
}
