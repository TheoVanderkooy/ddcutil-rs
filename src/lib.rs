//! Safe wrappers for the libddcutil C library.
//!
//! Prerequisites:
//!  - Version 2.1.x of `ddcutil` must be installed
//!  - Building requires `pkg-config` to locate the `libddcutil` headers
//!  - `ddcutil` is linux-only
mod capabilities;
mod display;
mod display_info;
mod err;
mod feature_metadata;
mod macros;

pub mod sys;

// re-exports of wrapper types & functions from other submodules
pub use display::{Display, DisplayIdentifier, TableValue};
pub use display_info::{DisplayInfo, DisplayInfoList, DisplayPath, get_display_info_list};
pub use err::{DdcError, Result};
pub use feature_metadata::{FeatureMetadata, FeatureValue};

// Re-exports of trivial bindgen structs that don't need wrapping
pub type MccsVersion = sys::DDCA_MCCS_Version_Spec;
pub type DdcutilVersion = sys::DDCA_Ddcutil_Version_Spec;

// Imports
use std::{ffi::CStr, ptr};

impl std::fmt::Display for MccsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}.{1}", self.major, self.minor)
    }
}

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

pub type SysLogLevel = sys::DDCA_Syslog_Level;
pub type LibInitOpts = sys::DDCA_Init_Options;
impl From<u32> for LibInitOpts {
    fn from(val: u32) -> Self {
        sys::DDCA_Init_Options(val)
    }
}

pub fn lib_init(libopts: Option<&str>, log_level: SysLogLevel, opts: LibInitOpts) -> Result<()> {
    unsafe {
        let rc = sys::ddca_init(
            libopts
                .map(|o| o.as_ptr() as *const i8)
                .unwrap_or(ptr::null()),
            log_level,
            opts,
        );
        DdcError::check(rc)?;
    }

    // TODO consider init2 with array-of-strings out-arg

    Ok(())
}

/// Turn verification of setting VCP values on/off. Returns previous setting.
pub fn lib_set_verify(onoff: bool) -> bool {
    unsafe { sys::ddca_enable_verify(onoff) }
}

pub fn lib_is_verify_enabled() -> bool {
    unsafe { sys::ddca_is_verify_enabled() }
}

pub fn lib_set_dynamic_sleep(onoff: bool) -> bool {
    unsafe { sys::ddca_enable_dynamic_sleep(onoff) }
}

pub fn lib_is_dynamic_sleep_enabled() -> bool {
    unsafe { sys::ddca_is_dynamic_sleep_enabled() }
}

pub fn lib_set_udf(onoff: bool) -> bool {
    unsafe { sys::ddca_enable_udf(onoff) }
}

pub fn lib_is_udf_enabled() -> bool {
    unsafe { sys::ddca_is_udf_enabled() }
}

pub fn feature_name(code: u8) -> Option<&'static str> {
    unsafe {
        let n = sys::ddca_get_feature_name(code);
        if n.is_null() {
            return None;
        }

        CStr::from_ptr(n).to_str().ok()
    }
}

pub fn get_feature_metadata(code: u8, version: MccsVersion) -> Result<FeatureMetadata> {
    let mut fm = FeatureMetadata(ptr::null_mut());
    unsafe {
        let rc = sys::ddca_get_feature_metadata_by_vspec(code, version, true, &mut fm.0);
        DdcError::check(rc)?;
    }

    Ok(fm)
}
