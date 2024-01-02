# libdtrace-rs

Rust bindings for DTrace.

### Requirements
- [Git for Windows](https://git-scm.com/download/win)
-  [Rust](https://www.rust-lang.org/tools/install)
- [Windows WDK and SDK](https://docs.microsoft.com/windows-hardware/drivers/download-the-wdk) (version 1903 or later)

### Compiling
1. Open an administrative powershell and set the execution policy
```ps1
Set-ExecutionPolicy RemoteSigned –Scope Process
```
2. Run `cargo build`
