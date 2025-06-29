//! Safe wrappers for the libddcutil C library.
//!
//! Prerequisites:
//!  - Version 2.x of `ddcutil` must be installed
//!  - Building requires `pkg-config` to locate the `libddcutil` headers
//!  - `ddcutil` is linux-only
//!
mod display_info;
mod err;
mod macros;

pub mod sys;

// re-exports of wrapper types & functions from other submodules
pub use display_info::{DisplayInfo, DisplayInfoList, get_display_info};
pub use err::{DdcError, Result};

// Re-exports of trivial bindgen structs that don't need wrapping
pub type MccsVersion = sys::DDCA_MCCS_Version_Spec;
pub type DdcutilVersion = sys::DDCA_Ddcutil_Version_Spec;

// Imports
use std::ffi::CStr;


/*
TODO:
 - setting vars uses display_handle
    - display_handle: from open_display, which takes a display_ref
    - display_ref: from get_display_ref, which take display_identifier, or get_display_refs, or get_display_info_list
    - display_identifier: from create_*_display_identifier:
        - display number
        - I2C (?) bus number
        - (manufacturer, model, serial #)
        - edid?
        - USB (bus, device) (both ints)
        - hiddev_devno (https://github.com/torvalds/linux/blob/master/Documentation/hid/hiddev.rst)
*/



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

// Features:
//   DDCA_Feature_List
//   DDCA_Feature_Value_Entry
//   DDCA_Feature_Metadata

// Capabilities:
//   DDCA_Cap_Vcp
//   DDCA_Capabilities

// ?
//   DDCA_Non_Table_Vcp_Value
//   DDCA_Table_Vcp_Value
//   DDCA_Any_Vcp_Value

// ?
//   DDCA_Display_Status_Event

// Settings:
//   DDCA_DW_Settings



/*
Functions TODO still: (excluding free_xyz functions)

ddca_init (deprecated use v2 if 2.1+)
ddca_init2  (2.1+ ... can we dynamically enable it? (or disable))

ddca_enable_verify
ddca_is_verify_enabled
ddca_set_sleep_multiplier (deprecated)
ddca_get_sleep_multiplier (deprecated)
ddca_set_display_sleep_multiplier
ddca_get_current_display_sleep_multiplier
ddca_enable_dynamic_sleep (2.1+)
ddca_is_dynamic_sleep_enabled (2.1+)
ddca_set_fout
ddca_set_fout_to_default
ddca_set_ferr
ddca_set_ferr_to_default

ddca_start_capture
ddca_end_capture

ddca_get_output_level
ddca_set_output_level
ddca_output_level_name
ddca_syslog_level_from_name
ddca_reset_stats
ddca_show_stats
ddca_report_locks

ddca_get_display_refs
ddca_get_display_info

ddca_redetect_displays

ddca_create_dispno_display_identifier
ddca_create_busno_display_identifier
ddca_create_mfg_model_sn_display_identifier
ddca_create_edid_display_identifier
ddca_create_usb_display_identifier
ddca_create_usb_hiddev_display_identifier

ddca_did_repr
ddca_create_display_ref (deprecated use `get`)
ddca_get_display_ref
ddca_validate_display_ref (2.1+)
ddca_dref_repr
ddca_dbgrpt_display_ref

ddca_open_display2
ddca_close_display

ddca_dh_repr
ddca_display_ref_from_handle

ddca_get_capabilities_string
ddca_parse_capabilities_string  ddca_free_parsed_capabilities

ddca_feature_list_from_capabilities
ddca_get_mccs_version_by_dh

ddca_enable_udf
ddca_is_udf_enabled
ddca_dfr_check_by_dref
ddca_dfr_check_by_dh

ddca_get_feature_metadata_by_vspec
ddca_get_feature_metadata_by_dref
ddca_get_feature_metadata_by_dh

ddca_get_feature_name
ddca_get_simple_nc_feature_value_name_by_table
ddca_dbgrpt_feature_metadata
ddca_report_display_by_dref
ddca_feature_list_id_name
ddca_get_feature_list_by_dref
...
ddca_feature_list_contains
...

ddca_get_non_table_vcp_value
ddca_get_table_vcp_value
ddca_get_any_vcp_value_using_explicit_type
ddca_get_any_vcp_value_using_implicit_type
ddca_format_table_vcp_value_by_dref
ddca_format_non_table_vcp_value_by_dref
ddca_format_any_vcp_value_by_dref
ddca_set_non_table_vcp_value
ddca_set_table_vcp_value
ddca_set_any_vcp_value

ddca_get_profile_related_values
ddca_set_profile_related_values
ddca_register_display_status_callback (2.1+)
ddca_unregister_display_status_callback (2.1+)
ddca_display_event_class_name (2.1+)
ddca_display_event_type_name (2.1+)

ddca_start_watch_displays
ddca_stop_watch_displays (2.1+)
ddca_get_active_watch_classes
ddca_get_display_watch_settings (2.2+)
ddca_set_display_watch_settings (2.2+)
*/

// misc utility functions

pub fn ddcutil_version() -> DdcutilVersion {
    unsafe { sys::ddca_ddcutil_version() }
}

pub fn ddcutil_version_string() -> &'static str {
    unsafe {
        CStr::from_ptr(sys::ddca_ddcutil_version_string())
            .to_str()
            .expect("version was not valid UTF8")
    }
}

pub fn ddcutil_extended_version_string() -> &'static str {
    unsafe {
        CStr::from_ptr(sys::ddca_ddcutil_extended_version_string())
            .to_str()
            .expect("extended version was not valid UTF8")
    }
}

pub fn libddcutil_filename() -> &'static str {
    unsafe {
        CStr::from_ptr(sys::ddca_libddcutil_filename())
            .to_str()
            .expect("libddcutil filename was not valid UTF8")
    }
}

// TODO: convert flags to an enumset?
pub fn ddcutil_build_flags() -> u32 {
    unsafe { sys::ddca_build_options() as u32 }
}
