use crate::callbacks::buffered;

/// Represents a handle to a DTrace instance.
pub struct dtrace_hdl {
    handle: *mut crate::dtrace_hdl_t,
}

impl dtrace_hdl {
    /// Opens a DTrace instance with the specified version and flags.
    ///
    /// # Arguments
    ///
    /// * `version` - The DTrace version to use.
    /// * `flags` - Flags to control the behavior of the DTrace instance.
    ///
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
    /// # Arguments
    ///
    /// * `handler` - The handler function to be called for each buffered trace record.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the handler was set successfully, or an error code if the handler could
    /// not be set.
    pub fn dtrace_handle_buffered(
        &self,
        handler: crate::dtrace_handle_buffered_f,
    ) -> Result<(), i32> {
        let status: i32;
        unsafe {
            status =
                crate::dtrace_handle_buffered(self.handle, Some(buffered), std::ptr::null_mut());
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
    /// * `spec` - The probespec to associate with the program.
    /// * `flags` - Flags to control the compilation behavior.
    /// * `args` - Optional arguments to the DTrace program.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a reference to the compiled `dtrace_prog` if successful, or
    /// an error code if the program could not be compiled.
    pub fn dtrace_program_strcompile(&self, program: &str, spec: crate::dtrace_probespec, flags: u32, args: Option<Vec<String>>) -> Result<&crate::dtrace_prog, i32> {
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
            unsafe {Ok(&*prog)}
        } else {
            Err(self.dtrace_errno())
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
    /// Returns the error message as a `String`.
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
