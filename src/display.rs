use std::ffi::CStr;
use std::mem::MaybeUninit;
use std::ptr;
use std::ptr::slice_from_raw_parts;

use crate::DdcError;
use crate::DisplayInfo;
use crate::FeatureMetadata;
use crate::MccsVersion;
use crate::Result;
use crate::capabilities::DisplayCapabilities;
use crate::sys::DDCA_Non_Table_Vcp_Value;
use crate::sys::ddca_parse_capabilities_string;
use crate::sys::{self};

pub enum DisplayIdentifier<'a> {
    DisplayNumber(i32),
    I2cBus(i32),
    /// At least one of manufacturer, model, serial must be non-none
    SerialNumber {
        manufacturer: Option<&'a CStr>,
        model: Option<&'a CStr>,
        serial: Option<&'a CStr>,
    },
    // TODO EDID?
    UsbDevice {
        bus: i32,
        device: i32,
    },
    UsbHid(i32),
}

#[repr(transparent)]
pub struct TableValue(*mut sys::DDCA_Table_Vcp_Value);

impl TableValue {
    pub fn as_slice(&self) -> &[u8] {
        unsafe {
            let len = (*self.0).bytect as usize;
            if len == 0 {
                &[]
            } else {
                &*slice_from_raw_parts((*self.0).bytes, (*self.0).bytect as usize)
            }
        }
    }
}

impl Drop for TableValue {
    fn drop(&mut self) {
        unsafe {
            sys::ddca_free_table_vcp_value(self.0);
        }
    }
}

#[repr(transparent)]
pub(crate) struct SysDisplayRef(pub(crate) sys::DDCA_Display_Ref);

impl SysDisplayRef {
    #[allow(dead_code)]
    pub fn validate(&self, require_not_asleep: bool) -> Result<()> {
        unsafe {
            let rc = sys::ddca_validate_display_ref(self.0, require_not_asleep);
            DdcError::check(rc)
        }
    }
}

impl std::fmt::Display for SysDisplayRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = unsafe { CStr::from_ptr(sys::ddca_dref_repr(self.0)) };

        write!(f, "{}", repr.to_string_lossy())
    }
}

#[repr(transparent)]
struct SysDisplayIdentifier(sys::DDCA_Display_Identifier);

impl Drop for SysDisplayIdentifier {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                // ignore errors when freeing
                let _rc = sys::ddca_free_display_identifier(self.0);
            }
        }
    }
}

impl std::fmt::Display for SysDisplayIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = unsafe { CStr::from_ptr(sys::ddca_did_repr(self.0)) };
        write!(f, "{}", repr.to_string_lossy())
    }
}

#[derive(Debug)]
pub struct Display {
    dh: sys::DDCA_Display_Handle,
}

impl Display {
    /// Construct & open a display from a provided display reference pointer
    fn from_ref(dref: SysDisplayRef) -> Result<Self> {
        let mut dh: sys::DDCA_Display_Handle = ptr::null_mut();
        unsafe {
            let rc = sys::ddca_open_display2(dref.0, false, &mut dh);
            DdcError::check(rc)?;
        }

        Ok(Display { dh })
    }

    /// Construct & open a display from the provided display identifier
    pub fn from_identifier(id: DisplayIdentifier) -> Result<Self> {
        // create the display identifier for the library
        // SysDisplayIdentifier is an RAII wrapper to make sure the identifier gets freed
        let mut did = SysDisplayIdentifier(ptr::null_mut());
        unsafe {
            let rc = match id {
                DisplayIdentifier::DisplayNumber(dispno) => {
                    sys::ddca_create_dispno_display_identifier(dispno, &mut did.0)
                }
                DisplayIdentifier::I2cBus(busno) => {
                    sys::ddca_create_busno_display_identifier(busno, &mut did.0)
                }
                DisplayIdentifier::SerialNumber {
                    manufacturer,
                    model,
                    serial,
                } => sys::ddca_create_mfg_model_sn_display_identifier(
                    manufacturer.map(|s| s.as_ptr()).unwrap_or(ptr::null()) as *const i8,
                    model.map(|s| s.as_ptr()).unwrap_or(ptr::null()) as *const i8,
                    serial.map(|s| s.as_ptr()).unwrap_or(ptr::null()) as *const i8,
                    &mut did.0,
                ),
                DisplayIdentifier::UsbDevice { bus, device } => {
                    sys::ddca_create_usb_display_identifier(bus, device, &mut did.0)
                }
                DisplayIdentifier::UsbHid(dev) => {
                    sys::ddca_create_usb_hiddev_display_identifier(dev, &mut did.0)
                }
            };
            DdcError::check(rc)?;
        }

        // Get display ref for the identifier
        let mut dref = SysDisplayRef(ptr::null_mut());
        unsafe {
            let rc = sys::ddca_get_display_ref(did.0, &mut dref.0);
            DdcError::check(rc)?;
        }

        Self::from_ref(dref)
    }

    /// Construct & open a display from the provided display info
    pub fn from_display_info(info: &DisplayInfo) -> Result<Self> {
        Self::from_ref(info.dref())
    }

    #[allow(dead_code)]
    pub(crate) fn get_display_ref(&self) -> SysDisplayRef {
        unsafe { SysDisplayRef(sys::ddca_display_ref_from_handle(self.dh)) }
    }

    pub fn get_capabilities(&self) -> Result<DisplayCapabilities> {
        let mut cap_str = ptr::null_mut();
        let mut cap_parsed = DisplayCapabilities(ptr::null_mut());

        unsafe {
            let rc = sys::ddca_get_capabilities_string(self.dh, &mut cap_str);
            DdcError::check(rc)?;

            let rc = ddca_parse_capabilities_string(cap_str, &mut cap_parsed.0);
            DdcError::check(rc)?;
        }

        Ok(cap_parsed)
    }

    pub fn get_mccs_version(&self) -> Result<MccsVersion> {
        unsafe {
            let mut mccs_v = sys::DDCA_VSPEC_UNKNOWN;

            let rc = sys::ddca_get_mccs_version_by_dh(self.dh, &mut mccs_v);
            DdcError::check(rc)?;

            Ok(mccs_v)
        }
    }

    pub fn check_dfr(&self) -> Result<()> {
        unsafe {
            let rc = sys::ddca_dfr_check_by_dh(self.dh);
            DdcError::check(rc)
        }
    }

    pub fn get_feature_metadata(&self, code: u8) -> Result<FeatureMetadata> {
        let mut fm = FeatureMetadata(ptr::null_mut());
        unsafe {
            let rc = sys::ddca_get_feature_metadata_by_dh(code, self.dh, true, &mut fm.0);
            DdcError::check(rc)?;
        }

        Ok(fm)
    }

    /// Get a 16-bit VCP value.
    ///
    /// Return value is a pair of (max value, current value)
    pub fn get_vcp_value(&self, code: sys::DDCA_Vcp_Feature_Code) -> Result<(u16, u16)> {
        let mut val: MaybeUninit<DDCA_Non_Table_Vcp_Value> = MaybeUninit::uninit();

        unsafe {
            let rc = sys::ddca_get_non_table_vcp_value(self.dh, code, val.as_mut_ptr());
            DdcError::check(rc)?;
        }

        let val = unsafe { val.assume_init() };

        // mh/ml are hi/lo bits of max value, sh/sl are hi/lo bits of current value
        Ok((
            u16::from_be_bytes([val.mh, val.ml]),
            u16::from_be_bytes([val.sh, val.sl]),
        ))
    }

    /// Set a 16-bit VCP value
    pub fn set_vcp_value(&self, code: sys::DDCA_Vcp_Feature_Code, value: u16) -> Result<()> {
        let [hi, lo] = value.to_be_bytes();

        unsafe {
            let rc = sys::ddca_set_non_table_vcp_value(self.dh, code, hi, lo);
            DdcError::check(rc)?;
        }
        Ok(())
    }

    /// Get a table value.
    pub fn get_vcp_table_value(&self, code: sys::DDCA_Vcp_Feature_Code) -> Result<TableValue> {
        let mut ret = TableValue(ptr::null_mut());

        unsafe {
            let rc = sys::ddca_get_table_vcp_value(self.dh, code, &mut ret.0);
            DdcError::check(rc)?;
        }

        Ok(ret)
    }

    /// Set a table value.
    pub fn set_vcp_table_value(&self, code: sys::DDCA_Vcp_Feature_Code, val: &[u8]) -> Result<()> {
        let mut new_val = sys::DDCA_Table_Vcp_Value {
            bytect: val.len() as u16,
            bytes: val.as_ptr() as *mut u8,
        };

        unsafe {
            let rc = sys::ddca_set_table_vcp_value(self.dh, code, &mut new_val);
            DdcError::check(rc)?;
        }

        Ok(())
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            // ignore possible errors from closing the display
            let _rc = sys::ddca_close_display(self.dh);
        }
    }
}

// TODO impl Display for Display with ddca_dh_repr?
