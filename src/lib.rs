use std::{ffi::CStr, fmt::Display, ptr::null_mut};

pub mod sys;

#[non_exhaustive]
#[derive(Debug)]
enum Status {
    Ok,
    DdcData,
    NullResponse,
    MultiPartReadFragment,
    AllTriesZero,
    ReportedUnsupported,
    ReadAllZero,
    Retries,
    Edid,
    ReadEdid,
    InvalidEdid,
    AllResponsesNull,
    DeterminedUnsupported,
    Arg,
    InvalidOperation,
    Unimplemented,
    Uninitialized,
    UnknownFeature,
    InterpretationFailed,
    MultiFeatureError,
    InvalidDisplay,
    InternalError,
    Other,
    Verify,
    NotFound,
    Locked,
    AlreadyOpen,
    BadData,
    ConfigError,
    InvalidConfigFile,
    Disconnected,
    DpmsAsleep,
    FLocked,
    Quiesced,

    // ...
    UNKNOWN,
}

impl Status {
    fn from_rc(rc: i32) -> Status {
        match rc {
            0 /*DDCRC_OK*/ => Status::Ok,
            sys::DDCRC_DDC_DATA => Status::DdcData,
            sys::DDCRC_NULL_RESPONSE => Status::NullResponse,
            sys::DDCRC_MULTI_PART_READ_FRAGMENT => Status::MultiPartReadFragment,
            sys::DDCRC_ALL_TRIES_ZERO => Status::AllTriesZero,
            sys::DDCRC_REPORTED_UNSUPPORTED => Status::ReportedUnsupported,
            sys::DDCRC_READ_ALL_ZERO => Status::ReadAllZero,
            sys::DDCRC_RETRIES => Status::Retries,
            sys::DDCRC_EDID => Status::Edid,
            sys::DDCRC_READ_EDID => Status::ReadEdid,
            sys::DDCRC_INVALID_EDID => Status::InvalidEdid,
            sys::DDCRC_ALL_RESPONSES_NULL => Status::AllResponsesNull,
            sys::DDCRC_DETERMINED_UNSUPPORTED => Status::DeterminedUnsupported,
            sys::DDCRC_ARG => Status::Arg,
            sys::DDCRC_INVALID_OPERATION => Status::InvalidOperation,
            sys::DDCRC_UNIMPLEMENTED => Status::Unimplemented,
            sys::DDCRC_UNINITIALIZED => Status::Uninitialized,
            sys::DDCRC_UNKNOWN_FEATURE => Status::UnknownFeature,
            sys::DDCRC_INTERPRETATION_FAILED => Status::InterpretationFailed,
            sys::DDCRC_MULTI_FEATURE_ERROR => Status::MultiFeatureError,
            sys::DDCRC_INVALID_DISPLAY => Status::InvalidDisplay,
            sys::DDCRC_INTERNAL_ERROR => Status::InternalError,
            sys::DDCRC_OTHER => Status::Other,
            sys::DDCRC_VERIFY => Status::Verify,
            sys::DDCRC_NOT_FOUND => Status::NotFound,
            sys::DDCRC_LOCKED => Status::Locked,
            sys::DDCRC_ALREADY_OPEN => Status::AlreadyOpen,
            sys::DDCRC_BAD_DATA => Status::BadData,
            // DDCRC_INVALID_CONFIG_FILE is an alias for DDCRC_CONFIG_ERROR
            sys::DDCRC_CONFIG_ERROR => Status::ConfigError,
            sys::DDCRC_DISCONNECTED => Status::Disconnected,
            sys::DDCRC_DPMS_ASLEEP => Status::DpmsAsleep,
            sys::DDCRC_FLOCKED => Status::FLocked,
            sys::DDCRC_QUIESCED => Status::Quiesced,
            _ => Status::UNKNOWN,
        }
    }
}

// TODO wrapper for DDCA_Error_Detail
#[derive(Debug)]
pub struct DdcError {
    rc: Status,
    // TODO return code? whole status enum might not be needed actually..
    name: &'static CStr,
    desc: &'static CStr,
    detail: *mut sys::DDCA_Error_Detail,
}

impl DdcError {
    /// TODO: note: assumes rc is valid!
    fn from_rc(rc: i32) -> Self {
        unsafe {
            let name = CStr::from_ptr(sys::ddca_rc_name(rc));
            let desc = CStr::from_ptr(sys::ddca_rc_desc(rc));
            let mut detail = sys::ddca_get_error_detail();

            // make sure it's the same error, and not a previous error that didn't get replaced
            if !detail.is_null() && (*detail).status_code != rc {
                detail = null_mut();
            }

            DdcError {
                rc: Status::from_rc(rc),
                name,
                desc,
                detail,
            }
        }
    }

    fn check(rc: i32) -> Result<()> {
        match rc {
            0 /*DDCRC_OK*/ => Ok(()),
            rc => Err(Self::from_rc(rc)),
        }
    }
}

impl Drop for DdcError {
    fn drop(&mut self) {
        if !self.detail.is_null() {
            unsafe {
                sys::ddca_free_error_detail(self.detail);
            }
        }
    }
}

// impl core::fmt::Debug for DdcError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("DdcError")
//             .field("rc", &self.rc)
//             .field("name", &self.name)
//             .field("desc", &self.desc)
//             .field("detail", &self.detail)
//                  // TODO better debug impl for detail?
//             .finish()

//         // todo!()
//     }
// }

impl Display for DdcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for DdcError {}

pub type Result<T> = std::result::Result<T, DdcError>;

#[repr(transparent)]
pub struct DisplayInfo(*mut sys::DDCA_Display_Info);

impl Drop for DisplayInfo {
    fn drop(&mut self) {
        unsafe {
            sys::ddca_free_display_info(self.0);
        }
    }
}

#[repr(transparent)]
pub struct DisplayInfoList(*mut sys::DDCA_Display_Info_List);

impl DisplayInfoList {
    fn as_slice(&self) -> &[DisplayInfo] {
        unsafe {
            std::slice::from_raw_parts(
                (*self.0).info.as_mut_ptr() as *const DisplayInfo,
                (*self.0).ct as usize,
            )
        }
    }
}

impl Drop for DisplayInfoList {
    fn drop(&mut self) {
        unsafe {
            sys::ddca_free_display_info_list(self.0);
        }
    }
}

impl<'a> IntoIterator for &'a DisplayInfoList {
    type Item = &'a DisplayInfo;
    type IntoIter = core::slice::Iter<'a, DisplayInfo>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}

pub fn get_display_info(include_invalid_displays: bool) -> Result<DisplayInfoList> {
    let mut ret: *mut sys::DDCA_Display_Info_List = std::ptr::null_mut();
    let rc = unsafe { sys::ddca_get_display_info_list2(include_invalid_displays, &mut ret) };
    DdcError::check(rc)?;

    Ok(DisplayInfoList(ret))
}

/*
Types:
    Display info
    Features
    Capabilities
    Status
    Settings

Enums:
    Status, Error_Detail
    Options
    Features
    IO mode
    ...

...
*/
