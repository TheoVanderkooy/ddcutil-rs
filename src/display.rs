use std::mem::MaybeUninit;
use std::ptr;

use crate::DdcError;
use crate::DisplayInfo;
use crate::Result;
use crate::sys::DDCA_Non_Table_Vcp_Value;
use crate::sys::{self};

pub enum DisplayIdentifier<'a> {
    DisplayNumber(i32),
    I2cBus(i32),
    /// At least one of manufacturer, model, serial must be non-none
    SerialNumber {
        manufacturer: Option<&'a str>,
        model: Option<&'a str>,
        serial: Option<&'a str>,
    },
    // TODO EDID?
    UsbDevice {
        bus: i32,
        device: i32,
    },
    UsbHid(i32),
}

#[repr(transparent)]
pub(crate) struct SysDisplayRef(pub(crate) sys::DDCA_Display_Ref);

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

#[derive(Debug)]
pub struct Display {
    // ...
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

    // TODO generalize the get/set VCP logic:
    //   - table vs non-table values
    //   - feature codes?
    //   - improve return value: contains "max" and "set" values (hi/lo)

    /// Get a 16-bit VCP value.
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
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            // ignore possible errors from closing the display
            let _rc = sys::ddca_close_display(self.dh);
        }
    }
}
