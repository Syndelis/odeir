use std::path::{Path, PathBuf};


fn main() {
    let ffi_file = "src/ffi/mod.rs";
    cxx_build::bridge(ffi_file)  // returns a cc::Build
        .compile("odeircpp");

    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or("target".to_string());
    let target_dir = Path::new(&target_dir);
    let cxx_dir = target_dir.join("cxxbridge");
    let ffi_file = cxx_dir.join("odeir").join(ffi_file.to_string() + ".h");
    let ffi_file = Path::new(&ffi_file);
    let include_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("include");
    std::fs::copy(ffi_file, include_dir.join("odeir.hpp")).unwrap();


    println!("cargo:rerun-if-changed=src/ffi/mod.rs");
}
