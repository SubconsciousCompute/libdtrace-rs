# libdtrace-rs

Rust bindings for DTrace.

### Requirements
- [Git for Windows](https://git-scm.com/download/win)
- [Rust](https://www.rust-lang.org/tools/install)
- [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
    - Choose "Desktop development with C++" while installing.
    - Add `C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\MSBuild\Current\Bin` to `PATH`

### Compiling
1. Setup requirements for [bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html)
2. Open an powershell and set the execution policy
```ps1
Set-ExecutionPolicy RemoteSigned –Scope Process
```
3. Run `cargo build`
