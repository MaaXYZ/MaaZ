#![allow(clippy::expect_used)]
#![allow(clippy::absolute_paths)]
#![allow(clippy::unwrap_used)]

fn main() {
    println!("cargo:rustc-link-search=./lib");

    println!("cargo:rustc-link-lib=MaaFramework");
    println!("cargo:rustc-link-lib=MaaRpc");
    println!("cargo:rustc-link-lib=MaaToolkit");

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .clang_arg("-Iinclude")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    tauri_build::build();
}
