// build.rs

fn main() {

    cxx_build::bridge("src/lib.rs")  // returns a cc::Build
        .include("include")
        .flag_if_supported("-std=c++20")
        .compile("odeir");

    println!("cargo:rerun-if-changed=src/lib.rs");
}