const FFI_SRC: &str = "src/ffi.rs";

fn main() {
    cxx_build::bridge(FFI_SRC) // returns a cc::Build
        .compile("odeircpp");

    println!("cargo:rerun-if-changed={}", FFI_SRC);
}
