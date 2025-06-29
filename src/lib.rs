mod display_info;
mod err;
mod macros;
pub mod sys;
pub use display_info::{DisplayInfo, DisplayInfoList, get_display_info};
pub use err::{DdcError, Result};

pub type MccsVersion = sys::DDCA_MCCS_Version_Spec;

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
