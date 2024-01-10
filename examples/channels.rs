use libdtrace_rs::*;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
static PROGRAM: &str = r#"
    syscall:::entry
    /pid != $pid/
    {
        printf("timestamp=%llu syscall_name=%s pid=%d process_name=%s \n", timestamp, probefunc, pid, execname);
    }
"#;

pub unsafe extern "C" fn buffered(
    bufdata: *const crate::dtrace_bufdata_t,
    arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let tx = & *(arg as *mut ::std::sync::mpsc::Sender<::std::string::String>);
    let msg = ::core::ffi::CStr::from_ptr((*bufdata).dtbda_buffered)
        .to_str()
        .expect("Failed to convert buffer to string");
    tx.send(msg.to_string()).unwrap();

    crate::DTRACE_HANDLE_OK as ::core::ffi::c_int
}

fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        let handle = wrapper::dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as i32, 0).unwrap();
        handle.dtrace_setopt("bufsize", "4m").unwrap();
        handle.dtrace_setopt("aggsize", "4m").unwrap();
        handle
            .dtrace_register_handler(crate::types::dtrace_handler::Buffered(Some(buffered)), Some(&tx as *const _ as *mut _))
            .unwrap();
        let prog = handle
            .dtrace_program_strcompile(
                PROGRAM,
                dtrace_probespec::DTRACE_PROBESPEC_NAME,
                DTRACE_C_ZDEFS,
                None,
            )
            .unwrap();
        handle.dtrace_program_exec(prog, None).unwrap();
        handle.dtrace_go().unwrap();
        println!("Waiting for data...");
        loop {
            handle.dtrace_sleep(); // Wait until new data is available
            handle
                .dtrace_work(None, Some(callbacks::chew), Some(callbacks::chew_rec), None)
                .unwrap_or(dtrace_workstatus_t::DTRACE_WORKSTATUS_OKAY);
        }
        handle.dtrace_stop().unwrap();
    });

    loop {
        let msg = rx.recv().unwrap();
        println!("Recieved: {}", msg);
    }
}