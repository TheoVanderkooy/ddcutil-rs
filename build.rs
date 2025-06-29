use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use pkg-config to find the library
    let library = pkg_config::probe_library("ddcutil")?;

    let include_args = library
        .include_paths
        .iter()
        .map(|p| format!("-I{}", p.to_string_lossy()));

    // generate & write bindings
    let bindings = bindgen::Builder::default()
        .clang_args(include_args)
        .header("src/sys/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // only ddcutil functions/types, not stdlib stuff
        .allowlist_item(r"(DDC|ddc)\w*_.*")
        .generate()
        .expect("Bindgen error!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings!");

    Ok(())
}
