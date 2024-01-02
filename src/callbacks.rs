use std::ffi::c_void;

pub unsafe extern "C" fn buffered(
    bufdata: *const crate::dtrace_bufdata_t,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let msg = ::core::ffi::CStr::from_ptr((*bufdata).dtbda_buffered)
        .to_str()
        .expect("Failed to convert buffer to string");
    println!("{}", msg);

    crate::DTRACE_HANDLE_OK as ::core::ffi::c_int
}