# libdtrace-rs

Rust bindings for DTrace.

## Requirements
- [Git for Windows](https://git-scm.com/download/win)
- [Rust](https://www.rust-lang.org/tools/install)
- [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022)
    - Choose "Desktop development with C++" while installing.
    - Add `C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\MSBuild\Current\Bin` to `PATH`

## Compiling
1. Setup requirements for [bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html)
2. Open an powershell and set the execution policy
```ps1
Set-ExecutionPolicy RemoteSigned –Scope Process
```
3. Run `cargo build`

## Running
In order to run examples and tests a few more steps are required.

1. Download [DTrace](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/dtrace#installing-dtrace-under-windows) kernel driver.
2. Enable tracing by running the following command in an elevated terminal
    ```
    bcdedit /set dtrace ON
    ```
3. Set the `_NT_SYMBOL_PATH` as follows
```
_NT_SYMBOL_PATH=srv*C:\symbols*https://msdl.microsoft.com/download/symbols
```
4. Create the symbols directory at `C:\symbols`
5. Run `cargo run --example <example_name>` to run an example or `cargo test` to run tests.
