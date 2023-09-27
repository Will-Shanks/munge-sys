use std::env;
#[cfg(feature="bindgen")]
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=munge");
    #[cfg(feature="static")]
    println!("cargo:rustc-link-search=static=munge");

    let munge_path = env::var("MUNGE_PATH").unwrap_or("/usr/lib".to_string());
    println!("cargo:rustc-link-search=native={}", munge_path);


    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    #[cfg(feature="bindgen")]
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    #[cfg(feature="bindgen")]
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    #[cfg(feature="bindgen")]
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

