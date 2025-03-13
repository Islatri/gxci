use std::ffi::CString;
use std::mem::size_of;
use std::thread::sleep;
use std::time::Duration;

use gxci::{
    raw::{gx_handle::*, gx_interface::*, gx_struct::*},
    utils::{builder::*, debug::print_device_info},
};

fn main() {
    // You can change the library path as you need
    let gx =
        GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll")
            .expect("Failed to load library");
    gx.gx_init_lib().expect("Failed to initialize library");

    // Update the device list
    let mut device_num = 0;
    gx.gx_update_device_list(&mut device_num, 1000)
        .expect("Failed to update device list");

    if device_num > 0 {
        let mut base_info: Vec<GX_DEVICE_BASE_INFO> = (0..device_num)
            .map(|_| GXDeviceBaseInfoBuilder::new().build())
            .collect();
        let mut size = (device_num as usize) * size_of::<GX_DEVICE_BASE_INFO>();
        let status = gx
            .gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size)
            .expect("Failed to get all device base info");

        if status == 0 {
            // Assuming 0 is GX_STATUS_SUCCESS
            println!(
                "Device base info retrieved successfully. Number of devices: {}",
                device_num
            );

            for device in &base_info {
                print_device_info(&device);
            }

            let mut device_handle: GX_DEV_HANDLE = std::ptr::null_mut();

            // Attempt to open the first device using its SN
            let first_device_sn = std::str::from_utf8(&base_info[0].szSN).unwrap_or("");

            let psz_content = CString::new(first_device_sn.trim_end_matches(char::from(0)))
                .expect("CString::new failed");

            let open_param = GXOpenParamBuilder::new()
                .psz_content(psz_content.as_ptr() as *const i8)
                .build();

            // Open the device with index 1,you can also open the device with other index
            let open_status = gx
                .gx_open_device(&open_param, &mut device_handle)
                .expect("Failed to open device with sn");

            if open_status == 0 {
                println!(
                    "Successfully opened device with first SN: {}",
                    first_device_sn.trim_end_matches(char::from(0))
                );

                sleep(Duration::from_secs(1));

                // Close the device
                gx.gx_close_device(device_handle)
                    .expect("Failed to close device");
                println!("Device closed.")
            } else {
                println!(
                    "Failed to open device with SN: {}",
                    first_device_sn.trim_end_matches(char::from(0))
                );
            }
        } else {
            println!("Failed to retrieve device base info, status: {}", status);
        }
    } else {
        println!("No Devices found.");
    }

    gx.gx_close_lib().expect("Failed to close library");
    println!("Library closed.")
}

// let first_device_sn = std::str::from_utf8(&base_info[0].szSN).unwrap_or("");
// let sn_c_str = CString::new(first_device_sn.trim_end_matches(char::from(0)))
//     .expect("CString::new failed");
// let sn_ptr = sn_c_str.as_ptr() as *const u8;
