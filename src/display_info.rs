use core::str;
use std::slice::SliceIndex;

use crate::MccsVersion;
use crate::display::SysDisplayRef;
use crate::err::*;
use crate::str_field_getter;
use crate::sys;

/// Location of the display.
#[non_exhaustive]
#[derive(Debug, PartialEq, Eq)]
pub enum DisplayPath {
    I2C { bus: i32 },
    USB { hiddev_devno: i32 },
}

#[repr(transparent)]
pub struct DisplayInfo(sys::DDCA_Display_Info);

impl DisplayInfo {
    pub fn display_no(&self) -> i32 {
        self.0.dispno as i32
    }

    pub fn path(&self) -> DisplayPath {
        let p = self.0.path;
        match p.io_mode {
            sys::DDCA_IO_Mode::DDCA_IO_I2C => DisplayPath::I2C {
                bus: unsafe { p.path.i2c_busno },
            },
            sys::DDCA_IO_Mode::DDCA_IO_USB => DisplayPath::USB {
                // TODO should this include the usb_devuce/bus fields of the display_info?
                hiddev_devno: unsafe { p.path.hiddev_devno },
            },
            #[allow(unreachable_patterns)]
            _ => panic!("Unknown IO mode {0:?}", p.io_mode),
        }
    }

    // TODO usb_bus/usb_device?

    str_field_getter!(manufacturer, mfg_id);

    str_field_getter!(model, model_name);

    str_field_getter!(serial_number, sn);

    pub fn product_code(&self) -> u16 {
        self.0.product_code
    }

    // TODO expose edid_bytes?

    pub fn vcp_version(&self) -> MccsVersion {
        self.0.vcp_version
    }

    pub(crate) fn dref(&self) -> crate::display::SysDisplayRef {
        SysDisplayRef(self.0.dref)
    }
}

#[repr(transparent)]
pub struct DisplayInfoList(*mut sys::DDCA_Display_Info_List);

impl DisplayInfoList {
    /// Convert to a slice of `DisplayInfo`.
    pub fn as_slice(&self) -> &[DisplayInfo] {
        unsafe {
            std::slice::from_raw_parts(
                (*self.0).info.as_mut_ptr() as *const DisplayInfo,
                (*self.0).ct as usize,
            )
        }
    }
}

impl<I: SliceIndex<[DisplayInfo]>> std::ops::Index<I> for DisplayInfoList {
    type Output = <I as SliceIndex<[DisplayInfo]>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.as_slice()[index]
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

pub fn get_display_info_list(include_invalid_displays: bool) -> Result<DisplayInfoList> {
    let mut ret: *mut sys::DDCA_Display_Info_List = std::ptr::null_mut();
    unsafe {
        let rc = sys::ddca_get_display_info_list2(include_invalid_displays, &mut ret);
        DdcError::check(rc)?;
    }

    Ok(DisplayInfoList(ret))
}
