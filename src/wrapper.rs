/// Represents a handle to a DTrace instance.
pub struct dtrace_hdl {
    handle: *mut crate::dtrace_hdl_t,
}

impl dtrace_hdl {
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
    pub fn dtrace_open(version: i32, flags: i32) -> Result<Self, i32> {
        let handle: *mut crate::dtrace_hdl_t;
        let mut errp: i32 = 0;
        unsafe {
            handle = crate::dtrace_open(version, flags, &mut errp);
        }
        if !handle.is_null() {
            Ok(Self { handle })
        } else {
            Err(errp)
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
    pub fn dtrace_setopt(&self, option: &str, value: &str) -> Result<(), i32> {
        let option = std::ffi::CString::new(option).unwrap();
        let value = std::ffi::CString::new(value).unwrap();
        let status;
        unsafe {
            status = crate::dtrace_setopt(self.handle, option.as_ptr(), value.as_ptr());
        }
        if status == 0 {
            Ok(())
        } else {
            Err(self.dtrace_errno())
        }
    }

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
    ///         unsafe extern "C" fn buffered(
    ///            bufdata: *const dtrace_bufdata_t,
    ///            args: *mut c_void,
    ///         ) -> c_int
    ///     ```
    /// # Returns
    ///
    /// Returns `Ok(())` if the handler was set successfully, or an error code if the handler could
    /// not be set.
    pub fn dtrace_handle_buffered(
        &self,
        handler: crate::dtrace_handle_buffered_f,
        arg: Option<&mut ::core::ffi::c_void>,
    ) -> Result<(), i32> {
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
            Err(self.dtrace_errno())
        }
    }

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
    ) -> Result<(), i32> {
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
            Err(self.dtrace_errno())
        }
    }

    /// Starts the execution of the program.
    ///
    /// This action enables the specified probes. After `dtrace_go` function is called, the probes start to generate data.
    /// # Returns
    ///
    /// * `Ok(())` - If the program execution is successful.
    /// * `Err(errno)` - If the program execution fails. The error number (`errno`) is returned.
    pub fn dtrace_go(&self) -> Result<(), i32> {
        let status;
        unsafe {
            status = crate::dtrace_go(self.handle);
        }
        if status == 0 {
            Ok(())
        } else {
            Err(self.dtrace_errno())
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
        chew: crate::dtrace_consume_probe_f,
        chewrec: crate::dtrace_consume_rec_f,
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
        unsafe { crate::dtrace_work(self.handle, fp as *mut crate::FILE, chew, chewrec, arg) }
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
    pub fn dtrace_stop(&self) -> Result<(), String> {
        let status;
        unsafe {
            status = crate::dtrace_stop(self.handle);
        }
        if status != -1 {
            Err("Could not stop tracing".to_string())
        } else {
            Ok(())
        }
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

    /// Retrieves the current error number for the DTrace instance.
    ///
    /// # Returns
    ///
    /// Returns the current error number.
    pub fn dtrace_errno(&self) -> i32 {
        unsafe { crate::dtrace_errno(self.handle) }
    }
}

impl Drop for dtrace_hdl {
    fn drop(&mut self) {
        unsafe {
            crate::dtrace_close(self.handle);
        }
    }
}
