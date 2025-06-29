pub mod err;
pub mod sys;
use err::{DdcError, Result};

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
    unsafe {
        let rc = sys::ddca_get_display_info_list2(include_invalid_displays, &mut ret);
        DdcError::check(rc)?;
    }

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
