use std::path::{Path, PathBuf};

const FFI_SRC: &str = "src/ffi.rs";

fn main() {
    cxx_build::bridge(FFI_SRC) // returns a cc::Build
        .compile("odeircpp");

    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
    let target_dir = Path::new(&target_dir);
    let cxx_dir = target_dir.join("cxxbridge");
    let ffi_file = cxx_dir.join("odeir").join(FFI_SRC.to_string() + ".h");
    let ffi_file = Path::new(&ffi_file);
    let include_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("include");

    println!("cargo:rerun-if-changed={}", FFI_SRC);
}
