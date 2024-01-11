use libdtrace_rs::*;

pub unsafe extern "C" fn custom_callback(
    data: *const dtrace_probedata_t,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let pd = &*(*data).dtpda_pdesc;
    let cpu_id = (*data).dtpda_cpu;

    let name = ::core::ffi::CStr::from_ptr(pd.dtpd_name.as_ptr())
        .to_str()
        .expect("Failed to convert name to string");

    let function = ::core::ffi::CStr::from_ptr(pd.dtpd_func.as_ptr())
        .to_str()
        .expect("Failed to convert provider to string");

    println!("{:3} {:6} {:32}", "CPU", "ID", "FUNCTION:NAME");
    print!(
        "{:3} {:6} {:32}",
        cpu_id,
        pd.dtpd_id,
        format!("{}:{}", function, name)
    );

    DTRACE_CONSUME_THIS as ::core::ffi::c_int
}

fn main() -> Result<(), utils::Error> {
    let handle = wrapper::dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as i32, 0)?
        .dtrace_setopt("bufsize", "4m")?
        .dtrace_register_handler(
            crate::types::dtrace_handler::Buffered(Some(callbacks::buffered)),
            None,
        )?;
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
        types::dtrace_status::Ok => {
            handle
                .dtrace_consume(None, Some(custom_callback), Some(callbacks::chew_rec), None)
                .unwrap();
        }
        _ => {}
    }
    handle.dtrace_stop().unwrap();

    Ok(())
}
