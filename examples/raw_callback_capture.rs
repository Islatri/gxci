use std::mem::size_of;
use std::slice;
use std::thread::sleep;
use std::time::Duration;

use opencv::{core, highgui};

use gxci::{
    raw::{gx_enum::*, gx_handle::*, gx_interface::*, gx_struct::*},
    utils::{builder::GXDeviceBaseInfoBuilder, debug::print_device_info},
};

extern "C" fn frame_callback(p_frame_callback_data: *mut GX_FRAME_CALLBACK_PARAM) {
    // 避免刷屏
    // println!("Frame callback triggered.");
    // println!("Frame status: {:?}", unsafe { (*p_frame_callback_data).status });
    // println!("Frame All: {:?}", unsafe { *p_frame_callback_data });

    unsafe {
        let frame_callback_data = &*p_frame_callback_data;
        if frame_callback_data.status == 0 {
            let data = slice::from_raw_parts(
                frame_callback_data.pImgBuf as *const u8,
                (frame_callback_data.nWidth * frame_callback_data.nHeight) as usize,
            );
            let mat = core::Mat::new_rows_cols_with_data(
                frame_callback_data.nHeight,
                frame_callback_data.nWidth,
                // core::CV_8UC1,
                data,
            )
            .unwrap();
            highgui::imshow("Camera Frame", &mat).unwrap();
            if highgui::wait_key(10).unwrap() > 0 {
                highgui::destroy_window("Camera Frame").unwrap();
            }
        }
    }
}

fn main() -> Result<()> {
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

            // Attempt to open the first device using its SN
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

                gx.gx_set_float(device_handle, GX_FEATURE_ID::GX_FLOAT_GAIN, 20.0)?;

                let reg_result = gx.gx_register_capture_callback(device_handle, frame_callback);
                match reg_result {
                    Ok(_) => println!("Capture callback registered successfully."),
                    Err(e) => eprintln!("Failed to register capture callback: {:?}", e),
                }

                gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)
                    .expect("Failed to send command");

                highgui::named_window("Camera", highgui::WINDOW_AUTOSIZE).unwrap();
                loop {
                    sleep(Duration::from_secs(60));
                    break;
                }

                gx.gx_send_command(device_handle, GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)
                    .expect("Failed to send command");

                let unregeister_result = gx.gx_unregister_capture_callback(device_handle);
                match unregeister_result {
                    Ok(_) => println!("Capture callback unregistered successfully."),
                    Err(e) => eprintln!("Failed to unregister capture callback: {:?}", e),
                }

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
