use std::env;
use std::path::PathBuf;
use std::process::Command;

// Set-ExecutionPolicy RemoteSigned –Scope Process
// 'C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe' opendtrace.sln /t:dtrace_dll:Rebuild /p:Configuration=Release /p:Platform=x64

fn main() {
    // Tell cargo to tell rustc to link dtrace.lib
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", std::path::Path::new(&dir).join("target\\dtrace\\build\\x64\\Release\\lib").display());
    println!("cargo:rustc-link-lib=dtrace");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    Command::new("git")
        .args(&["clone", "https://github.com/microsoft/DTrace-on-Windows.git", "target\\dtrace"])
        .output()
        .expect("Failed to clone dtrace");

    // Build dtrace
    Command::new("powershell")
        .args(&[".\\build-dtrace.ps1"])
        .output()
        .expect("failed to get external tools");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h") // The input header
        .use_core() // Use core:: instead of std::
        .derive_debug(false) // Don't derive Debug for generated types
        .prepend_enum_name(false)
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        
        // Only generate bindings for dtrace
        .allowlist_var(".*(dt_.*|(?i)dtrace).*")
        .allowlist_type(".*(dt_.*|(?i)dtrace).*")
        .allowlist_function(".*(dt_.*|(?i)dtrace).*")
        
        // Include paths for dtrace
        .clang_arg("-Itarget\\dtrace\\lib\\libctf\\common")
        .clang_arg("-Itarget\\dtrace\\lib\\libdtrace\\common")
        .clang_arg("-Itarget\\dtrace\\lib\\libdtrace\\compat\\win32")
        .clang_arg("-Itarget\\dtrace\\lib\\libdtrace\\compat\\win32\\inc")
        
        .generate() // Generate the bindings.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
