extern crate bindgen;

use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=openvx");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let include_base = PathBuf::from("include").join("1.3.1");
    let standard_dir = include_base.join("standard");
    let extensions_dir = include_base.join("extensions");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg(format!("-I{}", standard_dir.to_str().unwrap()))
        .clang_arg(format!("-I{}", extensions_dir.to_str().unwrap()))
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        .detect_include_paths(true)
        // Create bindings for everything magical.
        .allowlist_function("vx.*")
        .allowlist_var("VX.*")
        .allowlist_type("vx.*")
        .allowlist_type("vxu.*")
        .allowlist_recursively(true)
        // Doesn't appear to work right now, but maybe some day ...
        .generate_comments(true)
        // Bonus flavors.
        .enable_cxx_namespaces()
        // Suppress linter warnings.
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_upper_case_globals)]")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src");
    bindings
        .write_to_file(out_path.join("lib.rs"))
        .expect("Couldn't write bindings!");
}
