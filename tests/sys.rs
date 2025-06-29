use libddcutil2::sys;
use std::{ptr, str};

#[cfg(test)]

fn as_str<'a>(arg: &'a [i8]) -> &'a str {
    unsafe { str::from_utf8(&*(arg as *const [i8] as *const [u8])).unwrap() }
}

#[test]
fn test_testing() {
    unsafe {
        let mut dlist_loc: *mut sys::DDCA_Display_Info_List = ptr::null_mut();
        let ret = sys::ddca_get_display_info_list2(false, &mut dlist_loc);

        println!("ret= {ret}");
        sys::ddca_report_display_info_list(dlist_loc, 2);

        let infos = (*dlist_loc).info.as_slice((*dlist_loc).ct as usize);

        for info in infos {
            println!("INFO THINGY:");
            println!("  marker={0:?}", as_str(&info.marker));
            println!("  dispno={0}", info.dispno);
            println!("  path="); //, info.path.io_mode, info.path.path); //mode0=I2C,1=USB
            match info.path.io_mode {
                0 => {
                    println!("    io_mode=I2C");
                    println!("    i2c_busno={0}", info.path.path.i2c_busno);
                }
                1 => {
                    println!("    io_mode=USB");
                    println!("    hiddev_devno={0}", info.path.path.hiddev_devno);
                }
                _ => println!("Unknown io mode {0}", info.path.io_mode),
            }
            println!("  usb_bus={0}", info.usb_bus);
            println!("  usb_device={0}", info.usb_device);
            println!("  mfg_id={0:?}", as_str(&info.mfg_id)); // manufacturer
            println!("  model_name={0:?}", as_str(&info.model_name)); // model
            println!("  sn={0:?}", as_str(&info.sn)); // serial number
            println!("  product_code={0}", info.product_code);
            // println!("  edid_bytes={0:?}", info.edid_bytes);
            println!(
                "  vcp_version={0}.{1}",
                info.vcp_version.major, info.vcp_version.minor
            );
            println!("  dref={0:?}", info.dref);
        }
    }

    panic!("fail test to see output");
}
