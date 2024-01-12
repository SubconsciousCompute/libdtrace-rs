pub unsafe extern "C" fn buffered(
    bufdata: *const crate::dtrace_bufdata_t,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let msg = ::core::ffi::CStr::from_ptr((*bufdata).dtbda_buffered)
        .to_str()
        .expect("Failed to convert buffer to string");
    print!("{}", msg);

    crate::DTRACE_HANDLE_OK as ::core::ffi::c_int
}

pub unsafe extern "C" fn chew(
    _data: *const crate::dtrace_probedata_t,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    crate::DTRACE_CONSUME_THIS as ::core::ffi::c_int
}

pub unsafe extern "C" fn chew_rec(
    _data: *const crate::dtrace_probedata_t,
    record: *const crate::dtrace_recdesc_t,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if record.is_null() {
        return crate::DTRACE_CONSUME_NEXT as ::core::ffi::c_int;
    }

    let action = (*record).dtrd_action;
    if action == crate::DTRACEACT_EXIT as u16 {
        return crate::DTRACE_CONSUME_NEXT as ::core::ffi::c_int;
    }

    crate::DTRACE_CONSUME_THIS as ::core::ffi::c_int
}

pub unsafe extern "C" fn walk(
    aggdata: *const crate::dtrace_aggdata_t,
    _arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let aggdesc = *(*aggdata).dtada_desc;
    let dtagd_rec = (aggdesc).dtagd_rec.as_ptr();
    let raw = (*aggdata).dtada_data as *mut u8;

    let nrec = *dtagd_rec.offset(0);
    let irec = *dtagd_rec.offset(1);

    let name = raw.offset((nrec.dtrd_size + nrec.dtrd_offset) as isize) as *const ::core::ffi::c_char;
    let name_str = ::core::ffi::CStr::from_ptr(name)
        .to_str()
        .expect("Failed to convert name to string");
    
    let instance = *(raw.offset((irec.dtrd_size + irec.dtrd_offset) as isize) as *const ::core::ffi::c_int);
    
    
    println!("{}\t\t\t\t\t\t{}", name_str, instance);

    return crate::DTRACE_AGGWALK_NEXT as ::core::ffi::c_int;
}
