use libdtrace_rs::*;

fn main() -> Result<(), utils::Error> {
    let handle = wrapper::dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as i32, 0)?
        .dtrace_setopt("bufsize", "4m")?
        .dtrace_setopt("aggsize", "4m")?
        .dtrace_register_handler(
            crate::types::dtrace_handler::Buffered(Some(callbacks::buffered)),
            None,
        )?;
    let prog = handle
        .dtrace_program_strcompile(
            "syscall:::entry { @num[execname] = count(); }",
            dtrace_probespec::DTRACE_PROBESPEC_NAME,
            DTRACE_C_ZDEFS,
            None,
        )
        .unwrap();
    handle.dtrace_program_exec(prog, None).unwrap();
    handle.dtrace_go().unwrap();

    for _ in 0..10 {
        handle.dtrace_sleep(); // Wait until new data is available
        handle
            .dtrace_work(None, Some(callbacks::chew), Some(callbacks::chew_rec), None)
            .unwrap();
    }

    handle.dtrace_aggregate_print(None, None).unwrap();
    handle.dtrace_stop().unwrap();

    Ok(())
}
