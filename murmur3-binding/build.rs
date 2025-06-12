fn main() {
    cc::Build::new()
        .file("murmur3_wrapper.c")
        .file("smhasher/src/MurmurHash3.cpp")
        .include("smasher")
        .cpp(true)
        .warnings(false)
        .compile("murmur3");

    println!("cargo:rerun-if-changed=murmur3_wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("murmur3_wrapper.h")
        .clang_arg("-Ismhasher")
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
