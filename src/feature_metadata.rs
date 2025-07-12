use std::{ffi::CStr, slice};

use crate::{
    MccsVersion,
    sys::{self},
};

pub struct FeatureSet(pub(crate) sys::DDCA_Feature_List);

impl FeatureSet {
    // TODO

    pub fn as_slice<'a>(&'a self) -> &'a [u8; 32] {
        &self.0.bytes
    }

    pub fn clear(&mut self) {
        unsafe {
            sys::ddca_feature_list_clear(&mut self.0);
        }
    }

    pub fn insert(&mut self, code: u8) -> &mut Self {
        unsafe {
            sys::ddca_feature_list_add(&mut self.0, code);
        }

        self
    }

    pub fn contains(&self, code: u8) -> bool {
        unsafe { sys::ddca_feature_list_contains(self.0, code) }
    }

    pub fn count(&self) -> i32 {
        unsafe { sys::ddca_feature_list_count(self.0) }
    }

    // TODO ddca_feature_list_string
}

impl PartialEq for FeatureSet {
    fn eq(&self, other: &Self) -> bool {
        unsafe { sys::ddca_feature_list_eq(self.0, other.0) }
    }
}

impl Eq for FeatureSet {}

impl std::ops::BitAnd for &FeatureSet {
    type Output = FeatureSet;

    fn bitand(self, rhs: Self) -> Self::Output {
        FeatureSet(unsafe { sys::ddca_feature_list_and(self.0, rhs.0) })
    }
}

impl std::ops::BitOr for &FeatureSet {
    type Output = FeatureSet;

    fn bitor(self, rhs: Self) -> Self::Output {
        FeatureSet(unsafe { sys::ddca_feature_list_or(self.0, rhs.0) })
    }
}

impl std::ops::Sub<&FeatureSet> for &FeatureSet {
    type Output = FeatureSet;

    fn sub(self, rhs: &FeatureSet) -> Self::Output {
        FeatureSet(unsafe { sys::ddca_feature_list_and_not(self.0, rhs.0) })
    }
}

#[repr(transparent)]
pub struct FeatureValue(sys::DDCA_Feature_Value_Entry);
impl<'a> FeatureValue {
    pub fn code(&self) -> u8 {
        self.0.value_code
    }

    pub fn name(&'a self) -> &'a str {
        unsafe {
            CStr::from_ptr(self.0.value_name)
                .to_str()
                .unwrap_or("<invalid name>")
        }
    }
}

#[repr(transparent)]
pub struct FeatureMetadata(pub(crate) *mut sys::DDCA_Feature_Metadata);

impl<'a> FeatureMetadata {
    pub fn feature_code(&self) -> u8 {
        unsafe { *self.0 }.feature_code
    }

    pub fn vcp_version(&self) -> MccsVersion {
        unsafe { *self.0 }.vcp_version
    }

    pub fn flags(&self) -> u16 {
        unsafe { *self.0 }.feature_flags
    }

    pub fn sl_values(&'a self) -> &'a [FeatureValue] {
        let sl_vals = unsafe { *self.0 }.sl_values;

        if sl_vals.is_null() {
            return &[];
        }

        // find the null terminator/array length
        let mut num = 0;
        while unsafe {
            let f = *(sl_vals.wrapping_add(num));
            !(f.value_code == 0 && f.value_name.is_null())
        } {
            num += 1;
        }

        // construct slice
        unsafe { slice::from_raw_parts(sl_vals as *const FeatureValue, num) }
    }

    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr((*self.0).feature_name) }
    }

    pub fn description(&self) -> &CStr {
        unsafe { CStr::from_ptr((*self.0).feature_desc) }
    }
}

impl Drop for FeatureMetadata {
    fn drop(&mut self) {
        unsafe {
            sys::ddca_free_feature_metadata(self.0);
        }
    }
}
