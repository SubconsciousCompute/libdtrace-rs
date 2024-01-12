use libdtrace_rs::*;

/*
    Created using excellent blog by Meelo: https://captmeelo.com/redteam/maldev/2022/05/10/ntcreateuserprocess.html
*/
static PROGRAM: &str = r#"
syscall::NtCreateUserProcess:entry
{
    this->ImagePathName = ((nt`_RTL_USER_PROCESS_PARAMETERS*)
        copyin(arg8, sizeof(nt`_RTL_USER_PROCESS_PARAMETERS)))->ImagePathName;

    this->fname = wstr2str((wchar_t*)
        copyin((uintptr_t)this->ImagePathName.Buffer,
                this->ImagePathName.Length), this->ImagePathName.Length/2);

    printf("Process %s PID %d created %s \n", execname, pid, this->fname);
}
"#;

fn main() -> Result<(), utils::Error> {
    let handle = wrapper::dtrace_hdl::dtrace_open(libdtrace_rs::DTRACE_VERSION as i32, 0)?;
    handle.dtrace_setopt("bufsize", "4m")?;
    handle.dtrace_setopt("aggsize", "4m")?;
    handle.dtrace_setopt("sympath", "C:/symbols")?;
    handle.dtrace_register_handler(
        crate::types::dtrace_handler::Buffered(Some(callbacks::buffered)),
        None,
    )?;
    let prog = handle.dtrace_program_strcompile(
        PROGRAM,
        dtrace_probespec::DTRACE_PROBESPEC_NAME,
        DTRACE_C_ZDEFS,
        None,
    )?;
    handle.dtrace_program_exec(prog, None)?;
    handle.dtrace_go()?;

    loop {
        handle.dtrace_sleep();
        match handle.dtrace_work(
            None,
            Some(libdtrace_rs::callbacks::chew),
            Some(libdtrace_rs::callbacks::chew_rec),
            None,
        ) {
            Ok(libdtrace_rs::dtrace_workstatus_t::DTRACE_WORKSTATUS_DONE) => break,
            Ok(_) | Err(_) => (),
        }
    }

    handle.dtrace_stop()?;
    Ok(())
}
