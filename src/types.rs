pub enum dtrace_aggwalk_order {
    /// No sorting, use the default order
    None,
    /// First sort by variable name, then for multiple aggregations sort by ascending value
    Sorted,
    /// First sort by variable name, then for multiple aggregations sort by key
    KeySorted,
    /// First sort by variable name, then for multiple aggregations sort by value (Same as `Sorted`)
    ValSorted,
    /// First sort by key, then for multiple aggregations sort by variable (aggregation ID)
    KeyVarSorted,
    /// First sort by value, then for multiple aggregations sort by variable (aggregation ID)
    ValVarSorted,
    /// Same as `KeySorted` but in reverse order
    KeyRevSorted,
    /// Same as `ValSorted` but in reverse order
    ValRevSorted,
    /// Same as `KeyVarSorted` but in reverse order
    KeyVarRevSorted,
    /// Same as `ValVarSorted` but in reverse order
    ValVarRevSorted,
}

#[repr(u32)]
pub enum dtrace_status {
    /// No Status
    None = crate::DTRACE_STATUS_NONE,
    /// Status OK
    Ok = crate::DTRACE_STATUS_OKAY,
    /// `exit()` was called, tacing stopped
    Exited = crate::DTRACE_STATUS_EXITED,
    /// Fill buffer full, tracing stopped
    Filled = crate::DTRACE_STATUS_FILLED,
    /// Tracing already stopped
    Stopped = crate::DTRACE_STATUS_STOPPED,
}

impl From<u32> for dtrace_status {
    fn from(value: u32) -> Self {
        match value {
            crate::DTRACE_STATUS_NONE => dtrace_status::None,
            crate::DTRACE_STATUS_OKAY => dtrace_status::Ok,
            crate::DTRACE_STATUS_EXITED => dtrace_status::Exited,
            crate::DTRACE_STATUS_FILLED => dtrace_status::Filled,
            crate::DTRACE_STATUS_STOPPED => dtrace_status::Stopped,
            _ => panic!("Invalid dtrace_status value"),
        }
    }
}

pub struct dtrace_eprobedesc {
    pub dtepd_epid: u32,
    pub dtepd_probeid: u32,
    pub dtepd_uarg: u64,
    pub dtepd_size: u32,
    pub dtepd_nrecs: i32,
    pub dtepd_rec: *const crate::dtrace_recdesc_t,
}

impl From<*mut crate::dtrace_eprobedesc> for dtrace_eprobedesc {
    fn from(value: *mut crate::dtrace_eprobedesc) -> Self {
        unsafe {
            let dtepd_epid = (*value).dtepd_epid;
            let dtepd_probeid = (*value).dtepd_probeid;
            let dtepd_uarg = (*value).dtepd_uarg;
            let dtepd_size = (*value).dtepd_size;
            let dtepd_nrecs = (*value).dtepd_nrecs;
            let dtepd_rec = (*value).dtepd_rec.as_ptr();
            dtrace_eprobedesc {
                dtepd_epid,
                dtepd_probeid,
                dtepd_uarg,
                dtepd_size,
                dtepd_nrecs,
                dtepd_rec,
            }
        }
    }
}

pub struct dtrace_aggdesc<'a> {
    pub dtagd_name: &'a str,
    pub dtagd_varid: i64,
    pub dtagd_flags: i32,
    pub dtagd_id: u32,
    pub dtagd_epid: u32,
    pub dtagd_size: u32,
    pub dtagd_nrecs: i32,
    pub dtagd_pad: u32,
    pub dtagd_rec: *const crate::dtrace_recdesc_t,
}

impl<'a> From<*mut crate::dtrace_aggdesc> for dtrace_aggdesc<'a> {
    fn from(value: *mut crate::dtrace_aggdesc) -> Self {
        unsafe {
            let dtagd_name = ::core::ffi::CStr::from_ptr((*value).dtagd_name)
                .to_str()
                .expect("Failed to convert name to string");
            let dtagd_varid = (*value).dtagd_varid;
            let dtagd_flags = (*value).dtagd_flags;
            let dtagd_id = (*value).dtagd_id;
            let dtagd_epid = (*value).dtagd_epid;
            let dtagd_size = (*value).dtagd_size;
            let dtagd_nrecs = (*value).dtagd_nrecs;
            let dtagd_pad = (*value).dtagd_pad;
            let dtagd_rec = (*value).dtagd_rec.as_ptr();
            dtrace_aggdesc {
                dtagd_name,
                dtagd_varid,
                dtagd_flags,
                dtagd_id,
                dtagd_epid,
                dtagd_size,
                dtagd_nrecs,
                dtagd_pad,
                dtagd_rec,
            }
        }
    }
}
