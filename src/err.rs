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

impl std::error::Error for DdcError {}

/// Custom way to convert DdcError to anyhow error.
///
/// This is needed because these errors are not Send/Sync,
/// so this can't be implemented automatically by anyhow,
/// but also we can't implement From<DdcError> for anyhow
/// because it could provide an implementation in the future...
#[cfg(feature = "anyhow")]
pub trait ConvertToAnyhow {
    type Output;

    fn anyhow(self) -> Self::Output;
}

#[cfg(feature = "anyhow")]
impl ConvertToAnyhow for DdcError {
    type Output = anyhow::Error;

    fn anyhow(self) -> anyhow::Error {
        anyhow::anyhow!("{}", self)
    }
}

#[cfg(feature = "anyhow")]
impl<T> ConvertToAnyhow for std::result::Result<T, DdcError> {
    type Output = std::result::Result<T, anyhow::Error>;

    fn anyhow(self) -> Self::Output {
        self.map_err(|e| e.anyhow())
    }
}

#[cfg(feature = "anyhow")]
mod test {
    #[cfg(test)]
    #[test]
    fn test_to_anyhow() {
        use crate::*;

        // Make sure the "anyhow" conversion for result compiles
        fn to_anyhow() -> anyhow::Result<()> {
            use crate::err::ConvertToAnyhow;

            Err(DdcError {
                rc: -1,
                name: c"test",
                desc: c"error for testing",
                detail: ptr::null_mut(),
            })
            .anyhow()
        }

        assert!(to_anyhow().is_err());
    }
}
