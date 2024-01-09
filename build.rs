use std::env;
use std::path::PathBuf;
use std::process::Command;

const DTRACE_SRC_DIR: &str = "_dtrace";

// Set-ExecutionPolicy RemoteSigned –Scope Process
// 'C:\Program Files\Microsoft Visual Studio\2022\Community\MSBuild\Current\Bin\MSBuild.exe' opendtrace.sln /t:dtrace_dll:Rebuild /p:Configuration=Release /p:Platform=x64
fn get_dtrace_libpath() -> PathBuf {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    std::path::Path::new(&dir)
        .join(DTRACE_SRC_DIR)
        .join("dtrace/build/x64/Release/lib")
}

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Tell cargo to tell rustc to link dtrace.lib
    println!("cargo:rustc-link-lib=dtrace");
    println!(
        "cargo:rustc-link-search=native={}",
        get_dtrace_libpath().display()
    );

    print!("Trying to buidld dtrace library... ");
    build_dtrace().unwrap();
    println!("\t....[DONE]");

    let outdir = std::path::Path::new(&env::var("OUT_DIR").unwrap()).join("dtrace.dll");
    std::fs::copy(get_dtrace_libpath().join("dtrace.dll"), outdir).expect("Failed to copy dll");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h") // The input header
        .use_core() // Use core:: instead of std::
        .derive_debug(false) // Don't derive Debug for generated types
        .prepend_enum_name(false)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
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

fn build_dtrace() -> anyhow::Result<()> {
    let output = if !PathBuf::from(DTRACE_SRC_DIR).is_dir() {
        Command::new("git")
            .args(&[
                "clone",
                "https://github.com/microsoft/DTrace-on-Windows.git",
                DTRACE_SRC_DIR,
            ])
            .output()
            .expect("Failed to clone dtrace");
    } else {
        Command::new("git")
            .arg("udpate")
            .current_dir(DTRACE_SRC_DIR)
            .output()
            .expect("Failed to update dtrace");
    };

    println!("> git update/clone {output:?}");

    let output = Command::new("powershell.exe")
        .arg("-F")
        .arg("./build-dtrace.ps1")
        .output()
        .expect("failed to get external tools");

    println!("> git clone {output:?}");

    Ok(())
}
