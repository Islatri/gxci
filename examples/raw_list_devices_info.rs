use gxci::{
    raw::{gx_interface::*, gx_struct::*},
    utils::{builder::GXDeviceBaseInfoBuilder, debug::print_device_info},
};
use std::mem::size_of;

fn main() {
    // You can change the library path as you need
    let gx =
        GXInstance::new("C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll")
            .expect("Failed to load library");
    gx.gx_init_lib().expect("Failed to initialize library");

    let mut device_num = 0;
    gx.gx_update_device_list(&mut device_num, 1000)
        .expect("Failed to update device list");

    if device_num > 0 {
        let mut base_info: Vec<GX_DEVICE_BASE_INFO> = (0..device_num)
            .map(|_| GXDeviceBaseInfoBuilder::new().build())
            .collect();

        // or you can use the following code to initialize the vector without using the builder

        // let mut base_info = vec![
        //     GX_DEVICE_BASE_INFO {
        //         szVendorName: [0; GX_INFO_LENGTH_32_BYTE],
        //         szModelName: [0; GX_INFO_LENGTH_32_BYTE],
        //         szSN: [0; GX_INFO_LENGTH_32_BYTE],
        //         szDisplayName: [0; GX_INFO_LENGTH_128_BYTE],
        //         szDeviceID: [0; GX_INFO_LENGTH_64_BYTE],
        //         szUserID: [0; GX_INFO_LENGTH_64_BYTE],
        //         accessStatus: GX_ACCESS_STATUS_CMD::Unknown,
        //         deviceClass: GX_DEVICE_CLASS::Unknown,
        //         reserved: [0; 300],
        //     };
        //     device_num as usize
        // ];

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
        } else {
            println!("Failed to retrieve device base info, status: {}", status);
        }
    } else {
        println!("No Devices found.");
    }

    gx.gx_close_lib().expect("Failed to close library");
    println!("Library closed.")
}
