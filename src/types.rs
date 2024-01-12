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

pub enum dtrace_handler {
    Buffered(crate::dtrace_handle_buffered_f),
    Drop(crate::dtrace_handle_drop_f),
    Err(crate::dtrace_handle_err_f),
    Proc(crate::dtrace_handle_proc_f),
    SetOpt(crate::dtrace_handle_setopt_f),
}
