use libddcutil2::*;
use std::{borrow::Cow, ffi::CStr, ptr};

#[cfg(test)]
#[test]
#[ignore]
fn test_testing() {
    unsafe {
        use std::mem::MaybeUninit;

        let feat = CStr::from_ptr(sys::ddca_get_feature_name(0x10))
            .to_str()
            .unwrap();
        println!("feat 0x10 = {feat}");

        let mut did: sys::DDCA_Display_Identifier = ptr::null_mut();
        let rc = sys::ddca_create_busno_display_identifier(6, &mut did);
        assert_eq!(rc, 0);
        let mut dref: sys::DDCA_Display_Ref = ptr::null_mut();
        let rc = sys::ddca_get_display_ref(did, &mut dref);
        assert_eq!(rc, 0);

        println!("display ref = {dref:?}");

        let mut refs = ptr::null_mut();
        sys::ddca_get_display_refs(false, &mut refs);

        let mut r = refs;
        loop {
            if (*r).is_null() {
                break;
            }
            println!("r: {0:?}  ref: {1:?}", r, *r);
            r = r.offset(1);
        }

        let mut dh = MaybeUninit::<sys::DDCA_Display_Handle>::zeroed().assume_init();
        let dh_ptr: *mut _ = &mut dh;
        let rc = sys::ddca_open_display2(dref, false, dh_ptr);
        assert_eq!(rc, 0);
        // TODO why does this not work??
        println!("Opened display!");

        // let mut dinfo = MaybeUninit::<sys::DDCA_Display_Info>::zeroed().assume_init();
        // let mut dinfo_ptr: *mut _ = &mut dinfo;

        // let rc = sys::ddca_get_display_info(dref, &mut dinfo_ptr);
        // assert_eq!(rc, 0);

        // let disp = Display::from_display_info(&*(dinfo_ptr as *mut DisplayInfo)).unwrap();
        // let val = disp.get_vcp_value(0x10).unwrap();

        let mut val = MaybeUninit::<_>::zeroed().assume_init();

        let rc = sys::ddca_get_non_table_vcp_value(dh, 0x10, &mut val);
        assert_eq!(rc, 0);

        println!("Value: {val:?}");

        // sys::ddca_set_non_table_vcp_value(dh, 0x10, 0, 100);

        sys::ddca_close_display(dh);
    }

    panic!(">>>>> fail test to see output <<<<<");
}

#[test]
#[ignore]
fn test_testing2() {
    // let x = 100u16.to_be_bytes();
    // println!("x={x:?}");

    println!("build flags = {0:?}", lib_build_flags());
    println!("testing = {0}", Cow::Borrowed("n/a"));

    panic!(">>>>> see output")
}
