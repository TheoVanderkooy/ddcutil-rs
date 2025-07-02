//! Safe wrappers for the libddcutil C library.
//!
//! Prerequisites:
//!  - Version 2.x of `ddcutil` must be installed
//!  - Building requires `pkg-config` to locate the `libddcutil` headers
//!  - `ddcutil` is linux-only
//!
mod display;
mod display_info;
mod err;
mod macros;

pub mod sys;

// re-exports of wrapper types & functions from other submodules
pub use display::Display;
pub use display_info::{DisplayInfo, DisplayInfoList, DisplayPath, get_display_info_list};
pub use err::{DdcError, Result};

// Re-exports of trivial bindgen structs that don't need wrapping
pub type MccsVersion = sys::DDCA_MCCS_Version_Spec;
pub type DdcutilVersion = sys::DDCA_Ddcutil_Version_Spec;

// Imports
use std::ffi::CStr;

// misc utility functions

pub fn lib_version() -> DdcutilVersion {
    unsafe { sys::ddca_ddcutil_version() }
}

pub fn lib_version_string() -> &'static str {
    unsafe {
        CStr::from_ptr(sys::ddca_ddcutil_version_string())
            .to_str()
            .expect("version was not valid UTF8")
    }
}

pub fn lib_extended_version_string() -> &'static str {
    unsafe {
        CStr::from_ptr(sys::ddca_ddcutil_extended_version_string())
            .to_str()
            .expect("extended version was not valid UTF8")
    }
}

pub fn lib_filename() -> &'static str {
    unsafe {
        CStr::from_ptr(sys::ddca_libddcutil_filename())
            .to_str()
            .expect("libddcutil filename was not valid UTF8")
    }
}

pub type BuildOptionFlags = sys::DDCA_Build_Option_Flags;

/// Returns a bitfield of `BuildOptionFlags`
pub fn lib_build_flags() -> BuildOptionFlags {
    unsafe { sys::ddca_build_options() }
}
