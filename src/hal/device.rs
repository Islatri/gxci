//! Device module for handling device operations.

use crate::error::{Error, ErrorKind, MutexExt, MutexType, Result};
use crate::hal::base::{GXI, gxi_check};
use crate::hal::check::{check_gx_status, check_gx_status_with_ok_fn};
use crate::raw::{gx_callback::*, gx_enum::*, gx_handle::*, gx_interface::*, gx_struct::*};
use crate::utils::builder::GXDeviceBaseInfoBuilder;
use crate::utils::extract::*;
use crate::utils::facade::*;
use crate::utils::imgproc::*;

use std::ffi::c_void;
use std::sync::{Arc, LazyLock, Mutex};
use std::thread::sleep;
use std::time::Duration;

#[cfg(feature = "use-imageproc")]
use imageproc::image::{ImageBuffer, Luma};
#[cfg(feature = "use-opencv")]
use opencv::{core, highgui, imgcodecs};

//----------------------------------------------------------
//---------------Common Functions---------------------------
//----------------------------------------------------------

pub fn gxi_check_device_handle() -> Result<()> {
    if GXI_DEVICE.lock_safe(MutexType::Device)?.is_none() {
        Err(Error::new(ErrorKind::DeviceHandleError(
            "Device handle is None. Please check your device open situation.".to_string(),
        )))
    } else {
        Ok(())
    }
}

pub fn gxi_get_device_handle() -> Result<*mut c_void> {
    gxi_check_device_handle()?;
    let gxi_device = GXI_DEVICE
        .lock_safe(MutexType::Device)?
        .as_ref()
        .ok_or(Error::new(ErrorKind::DeviceHandleError(
            "Device handle is None. Please check your device open situation.".to_string(),
        )))?
        .device;
    Ok(gxi_device)
}

pub fn gxi_count_devices(timeout: u32) -> Result<u32> {
    // Use `gxi_check` to ensure GXI is initialized and accessible
    let mut device_num = 0;
    gxi_check(|gxi| {
        gxi.gx_update_device_list(&mut device_num, timeout)?;
        Ok(())
    })?;
    Ok(device_num)
}

pub fn gxi_list_devices() -> Result<Vec<GX_DEVICE_BASE_INFO>> {
    let mut device_num = 0;
    gxi_check(|gxi| {
        gxi.gx_update_device_list(&mut device_num, 1000)?;
        Ok(())
    })?;

    let mut base_info: Vec<GX_DEVICE_BASE_INFO> = (0..device_num)
        .map(|_| GXDeviceBaseInfoBuilder::new().build())
        .collect();

    let mut size = (device_num as usize) * size_of::<GX_DEVICE_BASE_INFO>();

    // Populate the vector with device base information
    let status = GXI
        .lock_safe(MutexType::Gxi)?
        .as_ref()
        .ok_or_else(|| {
            Error::new(ErrorKind::GxiError(
                "GXI is None. Please check your gxci_init situation.".to_string(),
            ))
        })?
        .gx_get_all_device_base_info(base_info.as_mut_ptr(), &mut size)?;

    check_gx_status(status)?;
    Ok(base_info)
}

// //---------------Static LAMO  V-------------------------------

pub struct GxiDevice {
    pub device: GX_DEV_HANDLE,
}

unsafe impl Send for GxiDevice {}

pub static GXI_DEVICE: LazyLock<Arc<Mutex<Option<GxiDevice>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub struct GxiFrameData {
    pub frame_data: GX_FRAME_DATA,
    pub image_buffer: Vec<u8>,
}

unsafe impl Send for GxiFrameData {}

pub static GXI_FRAME_DATA: LazyLock<Arc<Mutex<Option<GxiFrameData>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub static GXI_IMAGE_BUFFER: LazyLock<Arc<Mutex<Option<Vec<u8>>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

#[cfg(feature = "solo")]
pub fn gxi_open_device() -> Result<()> {
    let mut device_num = 0;
    gxi_check(|gxi: &GXInstance| {
        gxi.gx_update_device_list(&mut device_num, 1000)?;
        Ok(())
    })?;

    let mut device = std::ptr::null_mut();
    let status = gxi_check(|gxi| gxi.gx_open_device_by_index(1, &mut device))?;

    check_gx_status_with_ok_fn(status, || {
        *GXI_DEVICE.lock_safe(MutexType::Device)? = Some(GxiDevice { device });
        Ok(())
    })?;
    println!("Successfully opened device");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_close_device() -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let status = gxi_check(|gxi| gxi.gx_close_device(gxi_device))?;

    check_gx_status_with_ok_fn(status, || {
        *GXI_DEVICE.lock_safe(MutexType::Device)? = None;
        Ok(())
    })?;
    println!("Successfully closed device");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_send_command(command: GX_FEATURE_ID) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let status = gxi_check(|gxi| gxi.gx_send_command(gxi_device, command))?;

    check_gx_status(status)?;
    println!("Successfully sent command");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_image() -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    println!("gxi_device: {:?}", gxi_device);

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)?;

    let byd = GXI.lock_safe(MutexType::Gxi)?;
    let gxi = byd.as_ref().ok_or(Error::new(ErrorKind::GxiError(
        "GXI is None. Please check your gxci_init situation.".to_string(),
    )))?;
    let device = GXI_DEVICE
        .lock_safe(MutexType::Device)?
        .as_ref()
        .ok_or(Error::new(ErrorKind::DeviceHandleError(
            "Device handle is None. Please check your device open situation.".to_string(),
        )))?
        .device;

    let (frame_data_facade, image_buffer) = fetch_frame_data(gxi, device)?;
    std::mem::drop(byd);
    let mut frame_data = convert_to_frame_data(&frame_data_facade);
    let status = gxi_check(|gxi| gxi.gx_get_image(gxi_device, &mut frame_data, 1000))?;
    *GXI_FRAME_DATA.lock_safe(MutexType::FrameData)? = Some(GxiFrameData {
        frame_data,
        image_buffer,
    });
    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)?;

    check_gx_status(status)?;
    println!("Successfully got image");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_get_image_as_frame_data() -> Result<GX_FRAME_DATA> {
    let gxi_device = gxi_get_device_handle()?;
    println!("gxi_device: {:?}", gxi_device);

    let byd = GXI.lock_safe(MutexType::Gxi)?;
    let gxi = byd.as_ref().ok_or(Error::new(ErrorKind::GxiError(
        "GXI is None. Please check your gxci_init situation.".to_string(),
    )))?;
    let device = GXI_DEVICE
        .lock_safe(MutexType::Device)?
        .as_ref()
        .ok_or(Error::new(ErrorKind::DeviceHandleError(
            "Device handle is None. Please check your device open situation.".to_string(),
        )))?
        .device;
    let (frame_data_facade, image_buffer) = fetch_frame_data(gxi, device)?;
    std::mem::drop(byd);
    let mut frame_data = convert_to_frame_data(&frame_data_facade);

    let status = gxi_check(|gxi| gxi.gx_get_image(gxi_device, &mut frame_data, 1000))?;

    *GXI_FRAME_DATA.lock_safe(MutexType::FrameData)? = Some(GxiFrameData {
        frame_data,
        image_buffer,
    });

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)?;

    check_gx_status(status)?;
    println!("Successfully got image");
    Ok(frame_data)
}

#[cfg(feature = "solo")]
pub fn gxi_get_image_as_raw() -> Result<&'static [u8]> {
    let gxi_device = gxi_get_device_handle()?;
    println!("gxi_device: {:?}", gxi_device);

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)?;

    let byd = GXI.lock_safe(MutexType::Gxi)?;
    let gxi = byd.as_ref().ok_or(Error::new(ErrorKind::GxiError(
        "GXI is None. Please check your gxci_init situation.".to_string(),
    )))?;
    let device = GXI_DEVICE
        .lock_safe(MutexType::Device)?
        .as_ref()
        .ok_or(Error::new(ErrorKind::DeviceHandleError(
            "Device handle is None. Please check your device open situation.".to_string(),
        )))?
        .device;

    let (frame_data_facade, image_buffer) = fetch_frame_data(gxi, device)?;
    std::mem::drop(byd);
    let mut frame_data = convert_to_frame_data(&frame_data_facade);

    let status = gxi_check(|gxi| gxi.gx_get_image(gxi_device, &mut frame_data, 1000))?;

    *GXI_FRAME_DATA.lock_safe(MutexType::FrameData)? = Some(GxiFrameData {
        frame_data,
        image_buffer,
    });

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)?;

    check_gx_status(status)?;
    println!("Successfully got image");

    let frame_data = GXI_FRAME_DATA
        .lock_safe(MutexType::FrameData)?
        .as_ref()
        .ok_or(Error::new(ErrorKind::FrameDataError(
            "Frame data is None. Please check your get image situation.".to_string(),
        )))?
        .frame_data;

    let raw = extract_img_buf(&frame_data);

    Ok(raw)
}

#[cfg(feature = "solo")]
pub fn gxi_get_image_as_bytes() -> Result<Vec<u8>> {
    let gxi_device = gxi_get_device_handle()?;
    println!("gxi_device: {:?}", gxi_device);

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)?;

    let byd = GXI.lock_safe(MutexType::Gxi)?;

    let gxi = byd.as_ref().ok_or(Error::new(ErrorKind::GxiError(
        "GXI is None. Please check your gxci_init situation.".to_string(),
    )))?;
    let device = GXI_DEVICE
        .lock_safe(MutexType::Device)?
        .as_ref()
        .ok_or(Error::new(ErrorKind::DeviceHandleError(
            "Device handle is None. Please check your device open situation.".to_string(),
        )))?
        .device;

    let (frame_data_facade, image_buffer) = fetch_frame_data(gxi, device)?;
    std::mem::drop(byd);
    let mut frame_data = convert_to_frame_data(&frame_data_facade);

    let status = gxi_check(|gxi| gxi.gx_get_image(gxi_device, &mut frame_data, 1000))?;

    *GXI_FRAME_DATA.lock_safe(MutexType::FrameData)? = Some(GxiFrameData {
        frame_data,
        image_buffer,
    });

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)?;

    check_gx_status(status)?;
    println!("Successfully got image");

    let frame_data = GXI_FRAME_DATA
        .lock_safe(MutexType::FrameData)?
        .as_ref()
        .ok_or(Error::new(ErrorKind::FrameDataError(
            "Frame data is None. Please check your get image situation.".to_string(),
        )))?
        .frame_data;

    let raw = extract_img_buf(&frame_data);

    Ok(raw.to_vec())
}

#[cfg(all(feature = "solo", feature = "use-opencv"))]
pub fn gxi_save_image_as_png(filename: &str) -> Result<()> {
    // let frame_data = GXI_FRAME_DATA.lock().map_err(|e| Error::new(ErrorKind::MutexPoisonOptionFrameDataError(e)))?.as_ref().unwrap().frame_data;
    let frame_data = GXI_FRAME_DATA
        .lock_safe(MutexType::FrameData)?
        .as_ref()
        .ok_or(Error::new(ErrorKind::FrameDataError(
            "Frame data is None. Please check your get image situation.".to_string(),
        )))?
        .frame_data;
    if frame_data.nStatus == 0 {
        if let Some(data) = extract_image_data(&frame_data) {
            let mat =
                core::Mat::new_rows_cols_with_data(frame_data.nHeight, frame_data.nWidth, &data)
                    .unwrap();
            let vec = core::Vector::<i32>::new();
            if imgcodecs::imwrite(filename, &mat, &vec).unwrap() {
                println!("Image saved successfully.");
            } else {
                println!("Failed to save the image.");
            }
        } else {
            println!("Failed to extract image data.");
        }
    }
    Ok(())
}

#[cfg(all(feature = "solo", feature = "use-imageproc"))]
pub fn gxi_save_image_as_png(filename: &str) -> Result<()> {
    let frame_data = GXI_FRAME_DATA.lock_safe(MutexType::FrameData)?.as_ref()
        ..ok_or(Error::new(ErrorKind::FrameDataError(
            "Frame data is None. Please check your get image situation.".to_string(),
        )))?
        .frame_data;

    if frame_data.nStatus == 0 {
        if let Some(data) = extract_image_data(&frame_data) {
            let img = ImageBuffer::<Luma<u8>, _>::from_raw(
                frame_data.nWidth as u32,
                frame_data.nHeight as u32,
                data,
            )
            .expect("Failed to create image buffer");

            if img.save(filename).is_ok() {
                println!("Image saved successfully.");
            } else {
                println!("Failed to save the image.");
            }
        } else {
            println!("Failed to extract image data.");
        }
    }
    Ok(())
}

//---------------Callback Fn-------------------------------

// Here have many try to streaming out.
// 1. LAMO - It's danger to high io and parallelism.
// 2. Channel - Anyway it need to be tryed.

pub static FRAME_CALLBACK_DATA: LazyLock<Arc<Mutex<Option<GxiFrameCallbackData>>>> =
    LazyLock::new(|| Arc::new(Mutex::new(None)));

pub struct GxiFrameCallbackData {
    pub frame_callback_data: GX_FRAME_CALLBACK_PARAM,
}

unsafe impl Send for GxiFrameCallbackData {}

#[cfg(all(feature = "solo", feature = "use-opencv"))]
extern "C" fn frame_callback_opencv(p_frame_callback_data: *mut GX_FRAME_CALLBACK_PARAM) {
    let frame_callback_data = extract_frame_callback_param(p_frame_callback_data);
    let data = extract_callback_img_buf(frame_callback_data);

    let mat = core::Mat::new_rows_cols_with_data(
        frame_callback_data.nHeight,
        frame_callback_data.nWidth,
        data,
    )
    .unwrap();

    highgui::imshow("Camera Frame", &mat).unwrap();
    if highgui::wait_key(10).unwrap() > 0 {
        highgui::destroy_window("Camera Frame").unwrap();
    }
}

#[cfg(feature = "solo")]
pub fn gxi_use_stream(frame_callback: GXCaptureCallBack) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let status = gxi_check(|gxi| gxi.gx_register_capture_callback(gxi_device, frame_callback))?;

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)?;
    highgui::named_window("Camera", highgui::WINDOW_AUTOSIZE).unwrap();
    loop {
        sleep(Duration::from_secs(10));
        break;
    }

    check_gx_status(status)?;
    println!("Successfully opened stream");
    Ok(())
}

#[cfg(all(feature = "solo", feature = "use-opencv"))]
pub fn gxi_open_stream() -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    let status =
        gxi_check(|gxi| gxi.gx_register_capture_callback(gxi_device, frame_callback_opencv))?;

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)?;
    highgui::named_window("Camera", highgui::WINDOW_AUTOSIZE).unwrap();
    loop {
        sleep(Duration::from_secs(10));
        break;
    }

    check_gx_status(status)?;
    println!("Successfully opened stream");
    Ok(())
}

#[cfg(feature = "solo")]
pub fn gxi_close_stream() -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)?;
    let status = gxi_check(|gxi| gxi.gx_unregister_capture_callback(gxi_device))?;

    check_gx_status(status)?;
    println!("Successfully closed stream");
    Ok(())
}

#[cfg(all(feature = "solo", feature = "use-opencv"))]
pub fn gxi_open_stream_interval(interval_secs: u64) -> Result<()> {
    let gxi_device = gxi_get_device_handle()?;
    gxi_check(|gxi| gxi.gx_register_capture_callback(gxi_device, frame_callback_opencv))?;
    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_START)?;

    highgui::named_window("Camera", highgui::WINDOW_AUTOSIZE).unwrap();
    loop {
        sleep(Duration::from_secs(interval_secs));
        break;
    }

    gxi_send_command(GX_FEATURE_ID::GX_COMMAND_ACQUISITION_STOP)?;
    let status = gxi_check(|gxi| gxi.gx_unregister_capture_callback(gxi_device))?;

    check_gx_status(status)?;
    println!("Successfully opened stream");
    Ok(())
}

//----------------------------------------------------------
//---------------Multi Camera-------------------------------
//----------------------------------------------------------
