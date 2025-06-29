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
        // Keep docs on ddcutil methods
        .clang_arg("-fretain-comments-from-system-headers")
        .header("src/sys/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // default to signed because all the error codes are negative
        .default_macro_constant_type(bindgen::MacroTypeVariation::Signed)
        // only ddcutil functions/types, not stdlib stuff
        .allowlist_item(r"(DDC|ddc)\w*_.*")
        .bitfield_enum("DDCA_Build_Option_Flags|DDCA_Init_Options|DDCA_Output_Level|DDCA_Stats_Type|DDCA_Capture_Option_Flags")
        .rustified_non_exhaustive_enum("DDCA_Syslog_Level|DDCA_Capture_Option_Flags|DDCA_IO_Mode")
        .rustified_enum("DDCA_Vcp_Value_Type")
        // 2.1+
        .rustified_non_exhaustive_enum("DDCA_Display_Event_Type|DDCA_Display_Event_Class")
        .generate()
        .expect("Bindgen error!");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("failed to write bindings!");

    Ok(())
}
