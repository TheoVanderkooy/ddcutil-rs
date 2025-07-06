
## object model changes
 - "display reference" type for things that can be done with non-opened displays
 - "open display" for display handles



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

// TODO these need drefs, figure out where these should go..
ddca_set_display_sleep_multiplier
ddca_get_current_display_sleep_multiplier

// These take FILEs
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

// invalidates existing display handles -- not safe to expose directly
ddca_redetect_displays

ddca_dbgrpt_display_ref

ddca_dh_repr





// to-implement after making display reference a proper type
ddca_dfr_check_by_dref
ddca_get_feature_metadata_by_dref


ddca_dbgrpt_feature_metadata
ddca_report_display_by_dref
ddca_feature_list_id_name
ddca_get_feature_list_by_dref


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