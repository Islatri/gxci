use gxci::hal::device::*;
use gxci::hal::base::*;
use gxci::hal::control::analog::*;
use gxci::hal::control::image_format::*;
use gxci::utils::debug::print_device_info;
use gxci::utils::extract::{extract_callback_img_buf,extract_frame_callback_param};
use gxci::raw::gx_struct::GX_FRAME_CALLBACK_PARAM;
use gxci::opencv::{core, highgui};


extern "C" fn frame_callback(p_frame_callback_data: *mut GX_FRAME_CALLBACK_PARAM) {
    
    let frame_callback_data = extract_frame_callback_param(p_frame_callback_data);
    let data = extract_callback_img_buf(frame_callback_data);

    let mat = core::Mat::new_rows_cols_with_data(
        frame_callback_data.nHeight, 
        frame_callback_data.nWidth, 
        data
    ).unwrap();
    
    highgui::imshow("Camera Frame", &mat).unwrap();
    if highgui::wait_key(10).unwrap() > 0 {
        highgui::destroy_window("Camera Frame").unwrap();
    }
}

fn main()->Result<()> {
    gxci_init_default()?;

    let device_num = gxi_count_devices( 1000)?;
    println!("Device number: {}", device_num);

    let base_info = gxi_list_devices()?;
    for device in &base_info {
        print_device_info(&device);
    }
    

    gxi_open_device()?;

    let max_h = gxi_get_max_height()?;
    let max_w = gxi_get_max_width()?;

    gxi_set_height(max_h)?;
    gxi_set_width(max_w)?;
    

    gxi_set_gain_auto_continuous()?;

    // gxi_set_gain(1.0)?;

    gxi_use_stream(frame_callback)?;

    gxi_close_device()?;

    gxci_close()?;

    Ok(())
}