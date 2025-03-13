use std::mem::size_of;
use std::slice;

use opencv::{core, imgcodecs};

use gxci::{
    raw::{gx_enum::*, gx_handle::*, gx_interface::*, gx_struct::*},
    utils::{builder::GXDeviceBaseInfoBuilder, debug::print_device_info, facade::*},
};

fn main() -> Result<()> {
    unsafe {
        let gx = GXInstance::new(
            "C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll",
        )
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
                println!(
                    "Device base info retrieved successfully. Number of devices: {}",
                    device_num
                );

                for device in &base_info {
                    print_device_info(&device);
                }

                let first_device_sn = std::str::from_utf8(&base_info[0].szSN).unwrap_or("");
                let mut device_handle: GX_DEV_HANDLE = std::ptr::null_mut();

                let open_status = gx
                    .gx_open_device_by_index(1, &mut device_handle)
                    .expect("Failed to open device with index");

                if open_status == 0 {
                    println!(
                        "Successfully opened device index 1 with SN: {}",
                        first_device_sn.trim_end_matches(char::from(0))
                    );

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)
                        .expect("Failed to send command");

                    // 这种写法在所有权机制下是错误的，因为image_buffer在返回的时候就已经被释放了
                    // let frame_data_facade = fetch_frame_data(&gx, device_handle);
                    // let mut frame_data = convert_to_frame_data(&frame_data_facade.unwrap());

                    // 这种写法是正确的，因为image_buffer被返回到了当前作用域
                    #[allow(unused_variables)]
                    let (frame_data_facade, image_buffer) =
                        fetch_frame_data(&gx, device_handle).unwrap();
                    let mut frame_data = convert_to_frame_data(&frame_data_facade);

                    let result = gx.gx_get_image(device_handle, &mut frame_data, 100);
                    match result {
                        Ok(_) => {
                            println!("Image captured successfully.");

                            if frame_data.nStatus == 0 {
                                let data = slice::from_raw_parts(
                                    frame_data.pImgBuf as *const u8,
                                    (frame_data.nWidth * frame_data.nHeight) as usize,
                                );

                                let mat = core::Mat::new_rows_cols_with_data(
                                    frame_data.nHeight,
                                    frame_data.nWidth,
                                    data,
                                )
                                .unwrap();

                                let vec = core::Vector::<i32>::new();
                                if imgcodecs::imwrite("right.png", &mat, &vec).unwrap() {
                                    println!("Image saved successfully.");
                                } else {
                                    println!("Failed to save the image.");
                                }
                            }
                        }
                        Err(e) => eprintln!("Failed to capture image: {:?}", e),
                    }

                    gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)
                        .expect("Failed to send command");

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
        println!("Library closed.");
        Ok(())
    }
}
