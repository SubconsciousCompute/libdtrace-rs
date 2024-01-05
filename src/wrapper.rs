#![allow(dead_code)]
use crate::utils::DtraceError;
use crate::types::dtrace_aggwalk_order;

/// Represents a handle to a DTrace instance.
pub struct dtrace_hdl {
    handle: *mut crate::dtrace_hdl_t,
}

impl From<*mut crate::dtrace_hdl_t> for dtrace_hdl {
    fn from(value: *mut crate::dtrace_hdl_t) -> Self {
        Self { handle: value }
    }
}

impl Drop for dtrace_hdl {
    fn drop(&mut self) {
        unsafe {
            crate::dtrace_close(self.handle);
        }
    }
}

unsafe impl Send for dtrace_hdl {}
unsafe impl Sync for dtrace_hdl {}

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

impl dtrace_hdl {
    /* General Purpose APIs BEGIN */
    /// Opens a DTrace instance with the specified version and flags.
    ///
    /// # Arguments
    ///
    /// * `version` - The DTrace version to use, `DTRACE_VERSION`. Specifying any version other than the current version causes DTrace to fail.
    /// * `flags` - Flags to control the behavior of the DTrace instance. Available flags:
    ///     * `DTRACE_O_NODEV` - Do not attempt to open any DTrace devices.
    ///     * `DTRACE_O_NOSYS` - Do not attempt to enable any DTrace providers.
    ///     * `DTRACE_O_LP64` - Force DTrace to operate in 64-bit mode.
    ///     * `DTRACE_O_ILP32` - Force DTrace to operate in 32-bit mode.
    /// # Returns
    ///
    /// Returns a `Result` containing the `dtrace_hdl` handle if successful, or an error code if
    /// the DTrace instance could not be opened.
    pub fn dtrace_open(version: i32, flags: i32) -> Result<Self, DtraceError> {
        let handle: *mut crate::dtrace_hdl_t;
        let mut errp: i32 = 0;
        unsafe {
            handle = crate::dtrace_open(version, flags, &mut errp);
        }
        if !handle.is_null() {
            Ok(handle.into())
        } else {
            Err(DtraceError::from(errp))
        }
    }

    /// Starts the execution of the program.
    ///
    /// This action enables the specified probes. After `dtrace_go` function is called, the probes start to generate data.
    /// # Returns
    ///
    /// * `Ok(())` - If the program execution is successful.
    /// * `Err(errno)` - If the program execution fails. The error number (`errno`) is returned.
    pub fn dtrace_go(&self) -> Result<(), DtraceError> {
        let status;
        unsafe {
            status = crate::dtrace_go(self.handle);
        }
        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /// Stops the DTrace data consumption.
    ///
    /// This function communicates to the kernel that this consumer no longer consumes data. The kernel disables any enabled probe and frees the memory for the buffers associated with this DTrace handle.
    ///
    /// If the consumer does not call the `dtrace_stop()` function, the kernel eventually performs the cleanup. The data gathering stops either when the `deadman` timer fires or when the DTrace device is closed. The buffers are freed when the device closes. The DTrace device closes either when the consumer calls the `dtrace_close()` function or when the consumer exits. It is best practice for the consumer to call the `dtrace_stop()` function.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the stop operation is successful.
    /// * `Err(String)` - If the stop operation fails. The error message is returned.
    pub fn dtrace_stop(&self) -> Result<(), DtraceError> {
        let status;
        unsafe {
            status = crate::dtrace_stop(self.handle);
        }
        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /// Pauses the DTrace consumer based on the interaction rates with the DTrace framework.
    ///
    /// The `dtrace_sleep()` function calculates the minimum amount of time a consumer needs to pause before it interacts with the DTrace framework again. This is determined based on three rates maintained by DTrace:
    ///
    /// * `switchrate` - Specifies how often the principal buffers must be consumed. Principal buffers are maintained as active and passive pairs per-CPU. The rate at which these buffers switch determines the `switchrate`.
    /// * `statusrate` - Specifies how often the consumer should check the DTrace status.
    /// * `aggrate` - Specifies how often the aggregation buffers are consumed. Aggregation buffers are not maintained as pairs in the same way as principal buffers.
    ///
    /// The function calculates the earliest time for it to wake up based on the last occurrence of these three events and their associated rates. If that earliest time is in the past, the function returns, otherwise it sleeps until that time.
    ///
    /// Note: You do not have to call the `dtrace_sleep()` function itself from a consumer. You can use the `dtrace_getopt()` function to get the values of the appropriate rate and use timers based on those values.
    pub fn dtrace_sleep(&self) {
        unsafe {
            crate::dtrace_sleep(self.handle);
        }
    }

    /// Retrieves the current error number for the DTrace instance.
    ///
    /// # Returns
    ///
    /// Returns the current error number.
    pub fn dtrace_errno(&self) -> i32 {
        unsafe { crate::dtrace_errno(self.handle) }
    }

    /// Retrieves the error message associated with the specified error number.
    ///
    /// # Arguments
    ///
    /// * `handle` - An optional handle to a DTrace instance. If `None`, the error message will be
    ///              retrieved for the global DTrace instance.
    /// * `errno` - The error number.
    ///
    /// # Returns
    ///
    /// Returns the error message as a [`String`].
    pub fn dtrace_errmsg(handle: Option<Self>, errno: i32) -> String {
        unsafe {
            let handle = match handle {
                Some(handle) => handle.handle,
                None => std::ptr::null_mut(),
            };
            let msg = crate::dtrace_errmsg(handle, errno);
            let msg = ::core::ffi::CStr::from_ptr(msg);
            let msg = msg.to_str().unwrap();
            msg.to_string()
        }
    }

    /// Sets a DTrace option to the specified value.
    ///
    /// # Arguments
    ///
    /// * `option` - The name of the option to set.
    /// * `value` - The value to set for the option.
    ///
    /// For a list of available options, see [DTrace Runtime Options](https://docs.oracle.com/en/operating-systems/oracle-linux/dtrace-v2-guide/dtrace_runtime_options.html).
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the option was set successfully, or an error code if the option could
    /// not be set.
    pub fn dtrace_setopt(&self, option: &str, value: &str) -> Result<(), DtraceError> {
        let option = std::ffi::CString::new(option).unwrap();
        let value = std::ffi::CString::new(value).unwrap();
        let status;
        unsafe {
            status = crate::dtrace_setopt(self.handle, option.as_ptr(), value.as_ptr());
        }
        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /* General Purpose APIs END */

    /* Programming APIs START */
    /// Compiles a DTrace program from a string representation.
    ///
    /// # Arguments
    ///
    /// * `program` - The DTrace program as a string.
    /// * `spec` - spec to indicate the context of the probe you are using.
    ///     * Available values can be found [here](https://docs.oracle.com/en/operating-systems/solaris/oracle-solaris/11.4/dtrace-guide/dtrace_program_strcompile-function.html)
    ///
    /// * `flags` - Flags to control the compilation behavior. Common flags:
    ///     * `DTRACE_C_ZDEFS` - Instructs the compiler to permit probes, whose definitions do not match the existing probes.
    ///                          By default, the compiler does not permit this.
    ///    *  `DTRACE_C_DIFV` - Shows the target language instructions that results from the compilation and additional information to execute the target language instructions.
    ///    *  `DTRACE_C_CPP` - Instructs the compiler to preprocess the input program with the C preprocessor.
    ///
    /// The full list of flags can be found [here](https://github.com/microsoft/DTrace-on-Windows/blob/0adebf25928264dffdc8240e850503865409f334/lib/libdtrace/common/dtrace.h#L115).
    /// * `args` - Optional arguments passed to the program.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a reference to the compiled `dtrace_prog` if successful, or
    /// an error code if the program could not be compiled.
    pub fn dtrace_program_strcompile(
        &self,
        program: &str,
        spec: crate::dtrace_probespec,
        flags: u32,
        args: Option<Vec<String>>,
    ) -> Result<&mut crate::dtrace_prog, i32> {
        let program = std::ffi::CString::new(program).unwrap();
        let (argc, argv) = match args {
            Some(args) => {
                let mut argv: Vec<*mut ::core::ffi::c_char> = Vec::new();
                for arg in args {
                    let arg = std::ffi::CString::new(arg).unwrap();
                    argv.push(arg.as_ptr() as *mut ::core::ffi::c_char);
                }
                (argv.len() as i32, argv.as_ptr())
            }
            None => (0, std::ptr::null()),
        };

        let prog;
        unsafe {
            prog = crate::dtrace_program_strcompile(
                self.handle,
                program.as_ptr(),
                spec,
                flags,
                argc,
                argv,
            );
        }

        if !prog.is_null() {
            unsafe { Ok(&mut *prog) }
        } else {
            Err(self.dtrace_errno())
        }
    }

    /// After the D program is compiled, this function is used to create the object file for the program and download the object file to the kernel.
    /// The object file contains all the information necessary for the DTrace framework in the kernel to execute the D program.
    ///
    /// # Arguments
    ///
    /// * `program` - A mutable reference to the data structure representing the compiled program. This is returned by the `dtrace_strcompile()` function.
    /// * `info` - An optional mutable reference to a variable, which contains information about the D program. The definition of the `dtrace_proginfo_t` can be found [`here`](https://github.com/microsoft/DTrace-on-Windows/blob/0adebf25928264dffdc8240e850503865409f334/lib/libdtrace/common/dtrace.h#L106).
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the program execution is successful.
    /// * `Err(errno)` - If the program execution fails. The error number (`errno`) is returned.
    pub fn dtrace_program_exec(
        &self,
        program: &mut crate::dtrace_prog,
        info: Option<&mut crate::dtrace_proginfo>,
    ) -> Result<(), DtraceError> {
        let status;
        let info = match info {
            Some(info) => info,
            None => std::ptr::null_mut(),
        };
        unsafe {
            status = crate::dtrace_program_exec(self.handle, program, info);
        }
        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /// Iterates over the statements associated with a D program, calling the specified function on each statement.
    ///
    /// # Arguments
    ///
    /// * `program` -  A mutable reference to the data structure representing the compiled program. This is returned by the `dtrace_strcompile()` function.
    /// * `handler` - The function to call on each statement.
    ///
    ///     The handler function must have the following signature:
    ///     ```rs
    ///     unsafe extern "C" fn( *mut dtrace_hdl_t, *mut dtrace_prog_t, *mut dtrace_stmtdesc_t, *mut c_void) -> c_int
    ///     ```
    /// * `arg` - An optional argument to be passed to the handler function. This argument can maintain any state between successive invocations of the handler function.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the iteration is successful.
    /// * `Err(errno)` - If the iteration fails. The error number (`errno`) is returned.
    pub fn dtrace_stmt_iter(
        &self,
        program: &mut crate::dtrace_prog,
        handler: crate::dtrace_stmt_f,
        arg: Option<*mut ::core::ffi::c_void>,
    ) -> Result<(), DtraceError> {
        let status;
        let arg = match arg {
            Some(arg) => arg,
            None => std::ptr::null_mut(),
        };
        unsafe {
            status = crate::dtrace_stmt_iter(self.handle, program, handler, arg);
        }
        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /* Programming APIs END */

    /* Data Consumption APIs START */
    /// Determines the status of the running DTrace instance.
    ///
    /// # Returns
    ///
    /// * `Ok(dtrace_status)` - If the status is successfully determined.
    /// * `Err(errno)` - If the status could not be determined.
    pub fn dtrace_status(&self) -> Result<dtrace_status, DtraceError> {
        let status;
        unsafe {
            status = crate::dtrace_status(self.handle);
        }

        if status != -1 {
            Ok(dtrace_status::from(status as u32))
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /// Consumes data from the principal buffers.
    ///
    /// # Arguments
    ///
    /// * `file` - An optional file handle for output.
    /// * `p_hldr` - A pointer to a function that processes an `enabling control block (ECB)`. An `ECB` is a clause from a D program associated with the enabled probe.
    /// * `r_hldr` - A pointer to a function that processes a records from the `ECB`.
    /// * `arg` - An optional argument to be passed to the `p_hldr` and `r_hldr` functions. This argument can maintain any state between successive invocations of the functions.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the consumption is successful.
    /// * `Err(errno)` - If the consumption fails. The error number (`errno`) is returned.
    pub fn dtrace_consume(
        &self,
        file: Option<std::fs::File>,
        p_hldr: crate::dtrace_consume_probe_f,
        r_hldr: crate::dtrace_consume_rec_f,
        arg: Option<*mut ::core::ffi::c_void>,
    ) -> Result<(), DtraceError> {
        use std::os::windows::io::AsRawHandle;
        let fp = match file {
            Some(file) => file.as_raw_handle(),
            None => std::ptr::null_mut(),
        };
        let arg = match arg {
            Some(arg) => arg,
            None => std::ptr::null_mut(),
        };
        let status;
        unsafe {
            status =
                crate::dtrace_consume(self.handle, fp as *mut crate::FILE, p_hldr, r_hldr, arg);
        }
        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /// Performs all of the work that must to be done periodically by a DTrace consumer.
    ///
    /// This function corresponds to the `statusrate`, `switchrate`, and `aggrate` rates. It first calls `dtrace_status()` to determine the status of the trace and then calls `dtrace_aggregate_snap()` and `dtrace_consume()` to consume any aggregation buffer or principal buffer data.
    ///
    /// # Arguments
    ///
    /// * `file` - An optional file handle for output.
    /// * `chew` - A function pointer that is called for each enabled probe ID (EPID) that is processed from the buffer.
    /// * `chewrec` - A function pointer that is called for each record that is processed for an EPID.
    /// * `arg` - An optional argument to be passed to the `chew` and `chewrec` functions. This argument can maintain any state between successive invocations of the functions.
    ///
    /// # Returns
    ///
    /// * `DTRACE_WORKSTATUS_OKAY` - If the work is successfully performed.
    /// * `DTRACE_WORKSTATUS_DONE` - If the work is done and no more work is expected.
    /// * `DTRACE_WORKSTATUS_ERROR` - If an error occurs while performing the work.
    pub fn dtrace_work(
        &self,
        file: Option<std::fs::File>,
        p_hldr: crate::dtrace_consume_probe_f,
        r_hldr: crate::dtrace_consume_rec_f,
        arg: Option<&mut ::core::ffi::c_void>,
    ) -> crate::dtrace_workstatus_t {
        use std::os::windows::io::AsRawHandle;
        let fp = match file {
            Some(file) => file.as_raw_handle(),
            None => std::ptr::null_mut(),
        };
        let arg = match arg {
            Some(arg) => arg,
            None => std::ptr::null_mut(),
        };
        unsafe { crate::dtrace_work(self.handle, fp as *mut crate::FILE, p_hldr, r_hldr, arg) }
    }

    /* Data Consumption APIs END */

    /* Handler APIs START */
    /// Sets a handler function for processing buffered trace data.
    ///
    /// If [`None`] is passed to `dtrace_work`, `dtrace_consume` or `dtrace_aggregate_print` function, then libdtrace makes use of the buffered I/O handler to process buffered trace data.
    /// # Arguments
    ///
    /// * `handler` - The handler function to be called for each buffered trace record.
    /// * `arg` - An optional argument to be passed to the handler function. This argument can maintain any state between successive invocations of the handler function.
    ///
    ///     The handler function must have the following signature:
    ///     ```rs
    ///     unsafe extern "C" fn(*const dtrace_bufdata_t, *mut c_void) -> c_int
    ///     ```
    /// # Returns
    ///
    /// Returns `Ok(())` if the handler was set successfully, or an error code if the handler could
    /// not be set.
    pub fn dtrace_handle_buffered(
        &self,
        handler: crate::dtrace_handle_buffered_f,
        arg: Option<&mut ::core::ffi::c_void>,
    ) -> Result<(), DtraceError> {
        let status: i32;
        let arg = match arg {
            Some(arg) => arg,
            None => std::ptr::null_mut(),
        };
        unsafe {
            status = crate::dtrace_handle_buffered(self.handle, handler, arg);
        }
        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /* Handler APIs END */

    /* Aggregation APIs START */
    /// Retrieves aggregation data from the kernel
    ///
    /// This function is called to transfer data from the in-kernel aggregation buffers to the userspace (consumer). The data is not processed at this point.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the aggregation data is successfully retrieved.
    /// * `Err(errno)` - If the aggregation data could not be retrieved. The error number (`errno`) is returned.
    pub fn dtrace_aggregate_snap(&self) -> Result<(), DtraceError> {
        let status;
        unsafe {
            status = crate::dtrace_aggregate_snap(self.handle);
        }
        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /// Processes DTrace aggregate data.
    ///
    /// The function can be passed a specific `walk()` function. If passed `None`, it defaults to the `dtrace_aggregate_walk_sorted()` function,
    /// and the callback function passed to the `walk()` function is the default function that the libdtrace library uses to print aggregate data.
    ///
    /// # Arguments
    ///
    /// * `file` - An optional file handle for output.
    /// * `handler` - A function pointer that is called for each aggregate buffer that is processed.
    /// * `arg` - An optional argument to be passed to the `handler` function. This argument can maintain any state between successive invocations of the function.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the processing is successful.
    /// * `Err(i32)` - If the processing fails. The error number is returned.
    pub fn dtrace_aggregate_print(
        &self,
        file: Option<std::fs::File>,
        handler: crate::dtrace_aggregate_walk_f,
    ) -> Result<(), DtraceError> {
        use std::os::windows::io::AsRawHandle;
        let fp = match file {
            Some(file) => file.as_raw_handle(),
            None => std::ptr::null_mut(),
        };
        let status;
        unsafe {
            status = crate::dtrace_aggregate_print(self.handle, fp as *mut crate::FILE, handler);
        }
        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /// Processes DTrace aggregate data.
    ///
    /// # Arguments
    ///
    /// * `handler` - A function pointer that is called for each aggregate buffer that is processed.
    /// * `arg` - An optional argument to be passed to the `handler` function. This argument can maintain any state between successive invocations of the function.
    /// * `order` - The order in which the data is processed. One of the members of the [`dtrace_aggwalk_order`] enum.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the processing is successful.
    /// * `Err(i32)` - If the processing fails. The error number is returned.
    pub fn dtrace_aggregate_walk(
        &self,
        handler: crate::dtrace_aggregate_f,
        arg: Option<*mut ::core::ffi::c_void>,
        order: dtrace_aggwalk_order,
    ) -> Result<(), DtraceError> {
        let status;
        let arg = match arg {
            Some(arg) => arg,
            None => std::ptr::null_mut(),
        };
        unsafe {
            status = match order {
                dtrace_aggwalk_order::None => {
                    crate::dtrace_aggregate_walk(self.handle, handler, arg)
                }
                dtrace_aggwalk_order::Sorted | dtrace_aggwalk_order::ValSorted => {
                    crate::dtrace_aggregate_walk_sorted(self.handle, handler, arg)
                }
                dtrace_aggwalk_order::KeySorted => {
                    crate::dtrace_aggregate_walk_keysorted(self.handle, handler, arg)
                }
                dtrace_aggwalk_order::KeyVarSorted => {
                    crate::dtrace_aggregate_walk_keyvarsorted(self.handle, handler, arg)
                }
                dtrace_aggwalk_order::ValVarSorted => {
                    crate::dtrace_aggregate_walk_valvarsorted(self.handle, handler, arg)
                }
                dtrace_aggwalk_order::KeyRevSorted => {
                    crate::dtrace_aggregate_walk_keyrevsorted(self.handle, handler, arg)
                }
                dtrace_aggwalk_order::ValRevSorted => {
                    crate::dtrace_aggregate_walk_valrevsorted(self.handle, handler, arg)
                }
                dtrace_aggwalk_order::KeyVarRevSorted => {
                    crate::dtrace_aggregate_walk_keyvarrevsorted(self.handle, handler, arg)
                }
                dtrace_aggwalk_order::ValVarRevSorted => {
                    crate::dtrace_aggregate_walk_valvarrevsorted(self.handle, handler, arg)
                }
            };
        }

        if status == 0 {
            Ok(())
        } else {
            Err(DtraceError::from(self.dtrace_errno()))
        }
    }

    /* Aggregation APIs END */
}
