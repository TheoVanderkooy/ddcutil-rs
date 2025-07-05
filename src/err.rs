use crate::sys;

use std::{borrow::Cow, ffi::CStr, fmt::Display, ptr::null_mut};

/// Results from this crate always use `DdcError` as errors
pub type Result<T> = std::result::Result<T, DdcError>;

/// Errors from the DDC library.
#[derive(Debug)]
pub struct DdcError {
    rc: i32,
    name: &'static CStr,
    desc: &'static CStr,
    detail: *mut sys::DDCA_Error_Detail,
}

impl DdcError {
    /// Construct a `DdcError` from a DDCA_Status return code.
    ///
    /// This will internally fetch details for the error, if applicable.
    ///
    /// Safety: `rc` argument should be a valid error code returned by one of the `sys` functions.
    /// Otherwise this may panic if it is out of the valid range of error codes.
    unsafe fn from_rc(rc: i32) -> Self {
        unsafe {
            let mut detail = sys::ddca_get_error_detail();
            let name = CStr::from_ptr(sys::ddca_rc_name(rc));
            let desc = CStr::from_ptr(sys::ddca_rc_desc(rc));

            // Make sure the details are for the same error, and not a previous error that didn't get replaced
            if !detail.is_null() && (*detail).status_code != rc {
                detail = null_mut();
            }

            DdcError {
                rc,
                name,
                desc,
                detail,
            }
        }
    }

    /// Convenience function for wrapping functions from `sys` that return error codes, to convert
    /// the result to `Result`.
    ///
    /// Safety: `rc` argument should be a valid error code returned by one of the `sys` functions.
    /// Otherwise this may panic if it is out of the valid range of error codes.
    pub(crate) unsafe fn check(rc: i32) -> Result<()> {
        match rc {
            sys::DDCRC_OK => Ok(()),
            rc => unsafe { Err(Self::from_rc(rc)) },
        }
    }

    /// Get the detail causes as a slice of pointers to the detail struct.
    fn detail_causes(&self) -> Option<&[*mut sys::DDCA_Error_Detail]> {
        let det = unsafe { self.detail.as_ref()? };

        if det.cause_ct == 0 {
            return None;
        }

        unsafe { Some(det.causes.as_slice(det.cause_ct as usize)) }
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

impl Display for DdcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Main error
        writeln!(
            f,
            "DDC Error: {0} ({1}): {2}",
            self.name.to_string_lossy(),
            self.rc,
            self.desc.to_string_lossy(),
        )?;

        // Details if present
        if self.detail.is_null() {
            return Ok(());
        }

        let det_desc = if let Some(x) = unsafe { (*self.detail).detail.as_ref() } {
            unsafe { CStr::from_ptr(x) }.to_string_lossy()
        } else {
            Cow::Borrowed("n/a")
        };

        writeln!(f, "  Detail: {0}", det_desc)?;

        // Causes if present in the details
        if let Some(causes) = self.detail_causes() {
            writeln!(f, "  Caused by:")?;
            for &c in causes {
                if c.is_null() {
                    continue;
                }
                unsafe {
                    let c_rc = (*c).status_code;
                    let det_desc = if let Some(x) = (*c).detail.as_ref() {
                        CStr::from_ptr(x).to_string_lossy()
                    } else {
                        Cow::Borrowed("n/a")
                    };
                    writeln!(
                        f,
                        "    {0} ({1}): {2}   Detail: {3}",
                        CStr::from_ptr(sys::ddca_rc_name(c_rc)).to_string_lossy(),
                        c_rc,
                        CStr::from_ptr(sys::ddca_rc_desc(c_rc)).to_string_lossy(),
                        det_desc,
                    )?;
                }
            }
        }

        Ok(())
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

impl std::error::Error for DdcError {}
