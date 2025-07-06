use std::borrow::Cow;
use std::ffi::CStr;
use std::slice;

use crate::feature_metadata::FeatureSet;
use crate::{MccsVersion, sys};

// TODO better name for htis?

#[repr(transparent)]
pub struct CapVcp(sys::DDCA_Cap_Vcp);

impl<'a> CapVcp {
    pub fn feature_code(&self) -> u8 {
        self.0.feature_code
    }

    pub fn values(&'a self) -> &'a [u8] {
        if self.0.values.is_null() || self.0.value_ct == 0 {
            return &[];
        }

        unsafe { slice::from_raw_parts(self.0.values, self.0.value_ct as usize) }
    }
}

#[repr(transparent)]
pub struct DisplayCapabilities(pub(crate) *mut sys::DDCA_Capabilities);

impl Drop for DisplayCapabilities {
    fn drop(&mut self) {
        unsafe {
            sys::ddca_free_parsed_capabilities(self.0);
        }
    }
}

impl<'a> DisplayCapabilities {
    pub fn version(&self) -> MccsVersion {
        unsafe { *self.0 }.version_spec
    }

    pub fn cmd_codes(&'a self) -> &'a [u8] {
        unsafe { slice::from_raw_parts((*self.0).cmd_codes, (*self.0).cmd_ct as usize) }
    }

    pub fn vcp_codes(&'a self) -> &'a [CapVcp] {
        if unsafe { (*self.0).vcp_codes.is_null() || (*self.0).vcp_code_ct == 0 } {
            return &[];
        }

        unsafe {
            slice::from_raw_parts(
                (*self.0).vcp_codes as *const CapVcp,
                (*self.0).vcp_code_ct as usize,
            )
        }
    }

    pub fn get_messages(&'a self) -> Vec<Cow<'a, str>> {
        if unsafe { (*self.0).messages.is_null() || (*self.0).msg_ct == 0 } {
            return vec![];
        }

        unsafe {
            slice::from_raw_parts((*self.0).messages, (*self.0).msg_ct as usize)
                .iter()
                .map(|&m| CStr::from_ptr(m).to_string_lossy())
                .collect()
        }
    }

    pub fn get_feature_bitfield(&self) -> FeatureSet {
        unsafe { FeatureSet(sys::ddca_feature_list_from_capabilities(&mut (*self.0))) }
    }
}
